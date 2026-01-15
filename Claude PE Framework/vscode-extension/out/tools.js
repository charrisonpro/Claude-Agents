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
Object.defineProperty(exports, "__esModule", { value: true });
exports.tools = void 0;
exports.executeTool = executeTool;
const fs = __importStar(require("fs"));
const path = __importStar(require("path"));
const AGENT_FILES_DIR = 'Agent Files';
// Tool definitions for Claude
exports.tools = [
    {
        name: 'read_file',
        description: 'Read the contents of a file in the PE Framework Files directory.',
        input_schema: {
            type: 'object',
            properties: {
                filename: {
                    type: 'string',
                    description: 'Relative path to the file'
                }
            },
            required: ['filename']
        }
    },
    {
        name: 'write_file',
        description: 'Write content to a file in the PE Framework Files directory.',
        input_schema: {
            type: 'object',
            properties: {
                filename: {
                    type: 'string',
                    description: 'Relative path for the file'
                },
                content: {
                    type: 'string',
                    description: 'Content to write to the file'
                }
            },
            required: ['filename', 'content']
        }
    },
    {
        name: 'list_files',
        description: 'List all markdown and text files in the PE Framework Files directory.',
        input_schema: {
            type: 'object',
            properties: {
                include_archive: {
                    type: 'boolean',
                    description: 'Include files from Arch/ directory'
                }
            }
        }
    },
    {
        name: 'init_project',
        description: 'Initialize a new project with Agent Files structure.',
        input_schema: {
            type: 'object',
            properties: {
                project_path: {
                    type: 'string',
                    description: 'Absolute path to the project directory'
                },
                project_name: {
                    type: 'string',
                    description: 'Name of the project'
                }
            },
            required: ['project_path', 'project_name']
        }
    },
    {
        name: 'list_project_files',
        description: 'List files in a project\'s Agent Files directory.',
        input_schema: {
            type: 'object',
            properties: {
                project_path: {
                    type: 'string',
                    description: 'Absolute path to the project directory'
                }
            },
            required: ['project_path']
        }
    },
    {
        name: 'read_project_file',
        description: 'Read a file from a project\'s Agent Files directory.',
        input_schema: {
            type: 'object',
            properties: {
                project_path: {
                    type: 'string',
                    description: 'Absolute path to the project directory'
                },
                filename: {
                    type: 'string',
                    description: 'Name of the file to read'
                }
            },
            required: ['project_path', 'filename']
        }
    },
    {
        name: 'write_project_file',
        description: 'Write content to a file in a project\'s Agent Files directory.',
        input_schema: {
            type: 'object',
            properties: {
                project_path: {
                    type: 'string',
                    description: 'Absolute path to the project directory'
                },
                filename: {
                    type: 'string',
                    description: 'Name of the file to write'
                },
                content: {
                    type: 'string',
                    description: 'Content to write to the file'
                }
            },
            required: ['project_path', 'filename', 'content']
        }
    }
];
// Tool execution
function executeTool(toolName, toolInput, frameworkPath) {
    try {
        switch (toolName) {
            case 'read_file': {
                const filename = toolInput.filename;
                const filePath = path.join(frameworkPath, filename);
                return fs.readFileSync(filePath, 'utf-8');
            }
            case 'write_file': {
                const filename = toolInput.filename;
                const content = toolInput.content;
                const filePath = path.join(frameworkPath, filename);
                const dir = path.dirname(filePath);
                if (!fs.existsSync(dir)) {
                    fs.mkdirSync(dir, { recursive: true });
                }
                fs.writeFileSync(filePath, content);
                return `Successfully wrote to ${filename}`;
            }
            case 'list_files': {
                const includeArchive = toolInput.include_archive || false;
                const files = listFilesRecursive(frameworkPath, frameworkPath, includeArchive);
                return `Files:\n${files.join('\n')}`;
            }
            case 'init_project': {
                const projectPath = toolInput.project_path;
                const projectName = toolInput.project_name;
                return initProject(projectPath, projectName);
            }
            case 'list_project_files': {
                const projectPath = toolInput.project_path;
                const files = listProjectFiles(projectPath);
                return `Project Files:\n${files.join('\n')}`;
            }
            case 'read_project_file': {
                const projectPath = toolInput.project_path;
                const filename = toolInput.filename;
                const filePath = path.join(projectPath, AGENT_FILES_DIR, filename);
                return fs.readFileSync(filePath, 'utf-8');
            }
            case 'write_project_file': {
                const projectPath = toolInput.project_path;
                const filename = toolInput.filename;
                const content = toolInput.content;
                const agentFilesDir = path.join(projectPath, AGENT_FILES_DIR);
                if (!fs.existsSync(agentFilesDir)) {
                    fs.mkdirSync(agentFilesDir, { recursive: true });
                }
                const filePath = path.join(agentFilesDir, filename);
                fs.writeFileSync(filePath, content);
                return `Successfully wrote to ${projectPath}/${AGENT_FILES_DIR}/${filename}`;
            }
            default:
                return `Unknown tool: ${toolName}`;
        }
    }
    catch (error) {
        return `Error: ${error instanceof Error ? error.message : String(error)}`;
    }
}
function listFilesRecursive(baseDir, currentDir, includeArchive) {
    const files = [];
    if (!fs.existsSync(currentDir)) {
        return files;
    }
    for (const entry of fs.readdirSync(currentDir, { withFileTypes: true })) {
        const fullPath = path.join(currentDir, entry.name);
        if (entry.isDirectory()) {
            if (entry.name === 'Arch' && !includeArchive) {
                continue;
            }
            files.push(...listFilesRecursive(baseDir, fullPath, includeArchive));
        }
        else if (entry.isFile()) {
            const ext = path.extname(entry.name).toLowerCase();
            if (ext === '.md' || ext === '.txt') {
                const relativePath = path.relative(baseDir, fullPath);
                files.push(relativePath);
            }
        }
    }
    return files.sort();
}
function listProjectFiles(projectPath) {
    const agentFilesDir = path.join(projectPath, AGENT_FILES_DIR);
    if (!fs.existsSync(agentFilesDir)) {
        throw new Error(`Agent Files directory not found at: ${agentFilesDir}`);
    }
    const files = [];
    for (const entry of fs.readdirSync(agentFilesDir, { withFileTypes: true })) {
        if (entry.isFile()) {
            files.push(entry.name);
        }
    }
    return files.sort();
}
function initProject(projectPath, projectName) {
    if (!fs.existsSync(projectPath)) {
        throw new Error(`Project path does not exist: ${projectPath}`);
    }
    const agentFilesDir = path.join(projectPath, AGENT_FILES_DIR);
    const status = [];
    // Create directory if needed
    if (!fs.existsSync(agentFilesDir)) {
        fs.mkdirSync(agentFilesDir, { recursive: true });
        status.push(`Created: ${agentFilesDir}`);
    }
    else {
        status.push(`Exists: ${agentFilesDir}`);
    }
    // Template files
    const templates = [
        ['Instructions.md', generateInstructionsTemplate(projectName)],
        ['Version_History.md', generateVersionHistoryTemplate(projectName)],
        ['Conventions.md', generateConventionsTemplate(projectName)],
        ['SME_Content.md', generateSMEContentTemplate(projectName)]
    ];
    for (const [filename, template] of templates) {
        const filePath = path.join(agentFilesDir, filename);
        if (fs.existsSync(filePath)) {
            status.push(`Exists (not modified): ${filename}`);
        }
        else {
            fs.writeFileSync(filePath, template);
            status.push(`Created: ${filename}`);
        }
    }
    return status.join('\n');
}
function generateInstructionsTemplate(projectName) {
    return `# ${projectName} - Instructions

## Overview
[Describe the purpose and goals of this project]

## Getting Started
[Steps to get started with this project]

## Key Features
- [Feature 1]
- [Feature 2]
- [Feature 3]

## Usage
[How to use this project]

## Notes
[Any additional notes or considerations]
`;
}
function generateVersionHistoryTemplate(projectName) {
    return `# ${projectName} - Version History

## Current Version: 0.1.0

### v0.1.0 - Initial Setup
- Initial project structure created
- Agent Files initialized

---

## Planned Updates
- [Future feature 1]
- [Future feature 2]
`;
}
function generateConventionsTemplate(projectName) {
    return `# ${projectName} - Conventions

## Naming Conventions
[Define naming standards for files, variables, etc.]

## File Organization
[Describe how files should be organized]

## Code Style
[Define code style guidelines if applicable]

## Documentation Standards
[How documentation should be written and maintained]

## Communication
[How to communicate updates and changes]
`;
}
function generateSMEContentTemplate(projectName) {
    return `# ${projectName} - Subject Matter Expert Content

## Domain Knowledge
[Key domain concepts and terminology]

## Business Rules
[Important business rules and logic]

## Reference Materials
[Links or references to external documentation]

## Expert Notes
[Notes from subject matter experts]

## FAQ
[Frequently asked questions and answers]
`;
}
