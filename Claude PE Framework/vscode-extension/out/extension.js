"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.activate = activate;
exports.deactivate = deactivate;
const vscode = __importStar(require("vscode"));
const sdk_1 = __importDefault(require("@anthropic-ai/sdk"));
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
const tools_1 = require("./tools");
let anthropic = null;
let conversationHistory = [];
let systemPrompt = '';
function activate(context) {
    // Register the webview provider
    const provider = new ChatViewProvider(context.extensionUri);
    context.subscriptions.push(vscode.window.registerWebviewViewProvider('claudePEAgent.chatView', provider));
    // Register command to open chat
    context.subscriptions.push(vscode.commands.registerCommand('claudePEAgent.openChat', () => {
        vscode.commands.executeCommand('claudePEAgent.chatView.focus');
    }));
}
function deactivate() { }
class ChatViewProvider {
    _extensionUri;
    _view;
    constructor(_extensionUri) {
        this._extensionUri = _extensionUri;
    }
    resolveWebviewView(webviewView, _context, _token) {
        this._view = webviewView;
        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [this._extensionUri]
        };
        webviewView.webview.html = this._getHtmlContent();
        // Handle messages from the webview
        webviewView.webview.onDidReceiveMessage(async (data) => {
            switch (data.type) {
                case 'sendMessage':
                    await this._handleUserMessage(data.message);
                    break;
                case 'clearHistory':
                    conversationHistory = [];
                    this._postMessage({ type: 'cleared' });
                    break;
            }
        });
        // Initialize on load
        this._initialize();
    }
    async _initialize() {
        const config = vscode.workspace.getConfiguration('claudePEAgent');
        const apiKey = config.get('apiKey') || process.env.ANTHROPIC_API_KEY;
        if (!apiKey) {
            this._postMessage({
                type: 'error',
                message: 'API key not set. Configure in settings or set ANTHROPIC_API_KEY environment variable.'
            });
            return;
        }
        anthropic = new sdk_1.default({ apiKey });
        // Load system prompt from PE Framework
        const frameworkPath = this._getFrameworkPath();
        if (frameworkPath) {
            const promptFile = path.join(frameworkPath, 'PE_Framework.md');
            if (fs.existsSync(promptFile)) {
                systemPrompt = fs.readFileSync(promptFile, 'utf-8');
                this._postMessage({ type: 'initialized', frameworkPath });
            }
            else {
                this._postMessage({
                    type: 'warning',
                    message: `PE Framework file not found at: ${promptFile}`
                });
            }
        }
    }
    _getFrameworkPath() {
        const config = vscode.workspace.getConfiguration('claudePEAgent');
        let frameworkPath = config.get('frameworkPath');
        if (!frameworkPath && vscode.workspace.workspaceFolders) {
            // Try to find PE Framework Files in workspace
            for (const folder of vscode.workspace.workspaceFolders) {
                const testPath = path.join(folder.uri.fsPath, 'PE Framework Files');
                if (fs.existsSync(testPath)) {
                    frameworkPath = testPath;
                    break;
                }
            }
        }
        return frameworkPath || null;
    }
    async _handleUserMessage(message) {
        if (!anthropic) {
            this._postMessage({
                type: 'error',
                message: 'API not initialized. Check your API key.'
            });
            return;
        }
        // Add user message to history
        conversationHistory.push({
            role: 'user',
            content: message
        });
        this._postMessage({ type: 'thinking' });
        try {
            const response = await this._chat();
            this._postMessage({
                type: 'response',
                message: response
            });
        }
        catch (error) {
            this._postMessage({
                type: 'error',
                message: `Error: ${error instanceof Error ? error.message : String(error)}`
            });
        }
    }
    async _chat() {
        if (!anthropic) {
            throw new Error('API not initialized');
        }
        const config = vscode.workspace.getConfiguration('claudePEAgent');
        const model = config.get('model') || 'claude-opus-4-5-20251101';
        const frameworkPath = this._getFrameworkPath() || '';
        const response = await anthropic.messages.create({
            model,
            max_tokens: 4096,
            system: systemPrompt,
            tools: tools_1.tools,
            messages: conversationHistory
        });
        // Add assistant response to history
        conversationHistory.push({
            role: 'assistant',
            content: response.content
        });
        // Check for tool use
        const hasToolUse = response.content.some((block) => block.type === 'tool_use');
        if (hasToolUse && response.stop_reason === 'tool_use') {
            // Execute tools and continue
            const toolResults = [];
            for (const block of response.content) {
                if (block.type === 'tool_use') {
                    this._postMessage({
                        type: 'toolUse',
                        tool: block.name
                    });
                    const result = (0, tools_1.executeTool)(block.name, block.input, frameworkPath);
                    toolResults.push({
                        type: 'tool_result',
                        tool_use_id: block.id,
                        content: result
                    });
                }
            }
            // Add tool results to history
            conversationHistory.push({
                role: 'user',
                content: toolResults
            });
            // Continue conversation
            return this._chat();
        }
        // Extract text response
        const textBlocks = response.content.filter((block) => block.type === 'text');
        return textBlocks.map((block) => block.text).join('\n');
    }
    _postMessage(message) {
        this._view?.webview.postMessage(message);
    }
    _getHtmlContent() {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Claude PE Agent</title>
    <style>
        * {
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }
        body {
            font-family: var(--vscode-font-family);
            font-size: var(--vscode-font-size);
            color: var(--vscode-foreground);
            background: var(--vscode-editor-background);
            height: 100vh;
            display: flex;
            flex-direction: column;
        }
        #chat-container {
            flex: 1;
            overflow-y: auto;
            padding: 12px;
        }
        .message {
            margin-bottom: 12px;
            padding: 10px 12px;
            border-radius: 8px;
            max-width: 90%;
            word-wrap: break-word;
        }
        .user {
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            margin-left: auto;
        }
        .assistant {
            background: var(--vscode-editor-inactiveSelectionBackground);
        }
        .system {
            background: var(--vscode-editorWarning-background);
            color: var(--vscode-editorWarning-foreground);
            font-size: 0.9em;
            text-align: center;
            max-width: 100%;
        }
        .tool-use {
            background: var(--vscode-textBlockQuote-background);
            font-size: 0.85em;
            padding: 6px 10px;
            border-left: 3px solid var(--vscode-textLink-foreground);
        }
        .thinking {
            color: var(--vscode-descriptionForeground);
            font-style: italic;
        }
        #input-container {
            padding: 12px;
            border-top: 1px solid var(--vscode-panel-border);
            display: flex;
            gap: 8px;
        }
        #message-input {
            flex: 1;
            padding: 8px 12px;
            border: 1px solid var(--vscode-input-border);
            background: var(--vscode-input-background);
            color: var(--vscode-input-foreground);
            border-radius: 4px;
            font-family: inherit;
            font-size: inherit;
            resize: none;
            min-height: 40px;
            max-height: 120px;
        }
        #message-input:focus {
            outline: 1px solid var(--vscode-focusBorder);
        }
        button {
            padding: 8px 16px;
            background: var(--vscode-button-background);
            color: var(--vscode-button-foreground);
            border: none;
            border-radius: 4px;
            cursor: pointer;
        }
        button:hover {
            background: var(--vscode-button-hoverBackground);
        }
        #clear-btn {
            background: var(--vscode-button-secondaryBackground);
            color: var(--vscode-button-secondaryForeground);
        }
        pre {
            background: var(--vscode-textCodeBlock-background);
            padding: 8px;
            border-radius: 4px;
            overflow-x: auto;
            margin: 8px 0;
        }
        code {
            font-family: var(--vscode-editor-font-family);
        }
    </style>
</head>
<body>
    <div id="chat-container"></div>
    <div id="input-container">
        <textarea id="message-input" placeholder="Type your message..." rows="1"></textarea>
        <button id="send-btn">Send</button>
        <button id="clear-btn">Clear</button>
    </div>

    <script>
        const vscode = acquireVsCodeApi();
        const chatContainer = document.getElementById('chat-container');
        const messageInput = document.getElementById('message-input');
        const sendBtn = document.getElementById('send-btn');
        const clearBtn = document.getElementById('clear-btn');

        function addMessage(content, type) {
            const div = document.createElement('div');
            div.className = 'message ' + type;

            // Basic markdown-like rendering
            let html = content
                .replace(/\`\`\`([\\s\\S]*?)\`\`\`/g, '<pre><code>$1</code></pre>')
                .replace(/\`([^\`]+)\`/g, '<code>$1</code>')
                .replace(/\\n/g, '<br>');

            div.innerHTML = html;
            chatContainer.appendChild(div);
            chatContainer.scrollTop = chatContainer.scrollHeight;
        }

        function sendMessage() {
            const message = messageInput.value.trim();
            if (!message) return;

            addMessage(message, 'user');
            messageInput.value = '';
            messageInput.style.height = 'auto';

            vscode.postMessage({ type: 'sendMessage', message });
        }

        sendBtn.addEventListener('click', sendMessage);

        messageInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                sendMessage();
            }
        });

        // Auto-resize textarea
        messageInput.addEventListener('input', () => {
            messageInput.style.height = 'auto';
            messageInput.style.height = Math.min(messageInput.scrollHeight, 120) + 'px';
        });

        clearBtn.addEventListener('click', () => {
            vscode.postMessage({ type: 'clearHistory' });
        });

        // Handle messages from extension
        window.addEventListener('message', (event) => {
            const data = event.data;
            switch (data.type) {
                case 'response':
                    // Remove thinking indicator
                    const thinking = chatContainer.querySelector('.thinking');
                    if (thinking) thinking.remove();
                    addMessage(data.message, 'assistant');
                    break;
                case 'thinking':
                    addMessage('Thinking...', 'thinking');
                    break;
                case 'toolUse':
                    addMessage('Using tool: ' + data.tool, 'tool-use');
                    break;
                case 'error':
                    addMessage('Error: ' + data.message, 'system');
                    break;
                case 'warning':
                    addMessage('Warning: ' + data.message, 'system');
                    break;
                case 'initialized':
                    addMessage('Connected to PE Framework at: ' + data.frameworkPath, 'system');
                    break;
                case 'cleared':
                    chatContainer.innerHTML = '';
                    addMessage('Conversation cleared', 'system');
                    break;
            }
        });
    </script>
</body>
</html>`;
    }
}
