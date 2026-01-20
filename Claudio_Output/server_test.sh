#!/bin/bash
# =============================================================================
# Server Setup Verification Script
# Run after server_setup.sh to verify all components installed correctly
# =============================================================================
# Run as: chmod +x server_test.sh && ./server_test.sh
# =============================================================================

echo "=============================================="
echo "  Server Setup Verification"
echo "=============================================="
echo ""

PASS=0
FAIL=0

# Function to test a command
test_command() {
    local name="$1"
    local cmd="$2"

    if eval "$cmd" > /dev/null 2>&1; then
        echo "✓ $name"
        ((PASS++))
    else
        echo "✗ $name - FAILED"
        ((FAIL++))
    fi
}

# Function to test a service
test_service() {
    local name="$1"

    if systemctl is-active --quiet "$name"; then
        echo "✓ $name service running"
        ((PASS++))
    else
        echo "✗ $name service - NOT RUNNING"
        ((FAIL++))
    fi
}

# Function to show version
show_version() {
    local name="$1"
    local cmd="$2"
    local version

    version=$(eval "$cmd" 2>/dev/null)
    if [ -n "$version" ]; then
        echo "  → $name: $version"
    fi
}

# -----------------------------------------------------------------------------
# Core Utilities
# -----------------------------------------------------------------------------
echo "--- Core Utilities ---"
test_command "git" "which git"
test_command "curl" "which curl"
test_command "wget" "which wget"
test_command "vim" "which vim"
test_command "tmux" "which tmux"
test_command "htop" "which htop"
test_command "jq" "which jq"
test_command "tree" "which tree"
test_command "unzip" "which unzip"
echo ""

# -----------------------------------------------------------------------------
# Security
# -----------------------------------------------------------------------------
echo "--- Security ---"
test_command "ufw installed" "which ufw"
test_command "ufw active" "sudo ufw status | grep -q 'Status: active'"
test_command "fail2ban installed" "which fail2ban-client"
test_service "fail2ban"
test_command "unattended-upgrades" "which unattended-upgrade"
echo ""

# -----------------------------------------------------------------------------
# Web Server
# -----------------------------------------------------------------------------
echo "--- Web Server ---"
test_command "nginx installed" "which nginx"
test_service "nginx"
test_command "nginx config valid" "sudo nginx -t"
echo ""

# -----------------------------------------------------------------------------
# Node.js
# -----------------------------------------------------------------------------
echo "--- Node.js ---"
test_command "node installed" "which node"
test_command "npm installed" "which npm"
test_command "pm2 installed" "which pm2"
show_version "Node.js" "node --version"
show_version "npm" "npm --version"
show_version "PM2" "pm2 --version"
echo ""

# -----------------------------------------------------------------------------
# Rust (check for ubuntu user)
# -----------------------------------------------------------------------------
echo "--- Rust ---"
if [ -f "/home/ubuntu/.cargo/bin/rustc" ]; then
    echo "✓ Rust installed for ubuntu user"
    ((PASS++))
    show_version "Rust" "/home/ubuntu/.cargo/bin/rustc --version"
    show_version "Cargo" "/home/ubuntu/.cargo/bin/cargo --version"
else
    # Check if running as ubuntu user
    if [ -f "$HOME/.cargo/bin/rustc" ]; then
        echo "✓ Rust installed"
        ((PASS++))
        show_version "Rust" "rustc --version"
        show_version "Cargo" "cargo --version"
    else
        echo "✗ Rust - NOT FOUND"
        ((FAIL++))
    fi
fi
echo ""

# -----------------------------------------------------------------------------
# Python
# -----------------------------------------------------------------------------
echo "--- Python ---"
test_command "python3 installed" "which python3"
test_command "pip3 installed" "which pip3"
show_version "Python" "python3 --version"

# Check for data science virtual environment
VENV_PATH="/opt/datascience-venv"
if [ -d "$VENV_PATH" ]; then
    echo "✓ Data science venv exists at $VENV_PATH"
    ((PASS++))

    echo "  Checking Python packages in venv..."
    packages=("numpy" "pandas" "scipy" "matplotlib" "scikit-learn" "jupyter")
    for pkg in "${packages[@]}"; do
        if $VENV_PATH/bin/python -c "import $pkg" 2>/dev/null; then
            echo "  ✓ $pkg"
            ((PASS++))
        else
            echo "  ✗ $pkg - NOT INSTALLED"
            ((FAIL++))
        fi
    done
else
    echo "✗ Data science venv - NOT FOUND at $VENV_PATH"
    ((FAIL++))
    echo "  (Run: sudo python3 -m venv /opt/datascience-venv)"
fi
echo ""

# -----------------------------------------------------------------------------
# R
# -----------------------------------------------------------------------------
echo "--- R ---"
test_command "R installed" "which R"
show_version "R" "R --version 2>/dev/null | head -1"

echo "  Checking R packages..."
r_packages=("tidyverse" "ggplot2" "dplyr" "jsonlite")
for pkg in "${r_packages[@]}"; do
    if R -e "library($pkg)" > /dev/null 2>&1; then
        echo "  ✓ $pkg"
        ((PASS++))
    else
        echo "  ✗ $pkg - NOT INSTALLED"
        ((FAIL++))
    fi
done
echo ""

# -----------------------------------------------------------------------------
# Databases
# -----------------------------------------------------------------------------
echo "--- Databases ---"
test_command "PostgreSQL installed" "which psql"
test_service "postgresql"
test_command "SQLite installed" "which sqlite3"
show_version "PostgreSQL" "psql --version"
show_version "SQLite" "sqlite3 --version"
echo ""

# -----------------------------------------------------------------------------
# Claude Code CLI
# -----------------------------------------------------------------------------
echo "--- Claude Code ---"
if which claude-code > /dev/null 2>&1 || which claude > /dev/null 2>&1; then
    echo "✓ Claude Code CLI installed"
    ((PASS++))
else
    echo "✗ Claude Code CLI - NOT FOUND"
    echo "  (May need: npm install -g @anthropic-ai/claude-code)"
    ((FAIL++))
fi
echo ""

# -----------------------------------------------------------------------------
# Firewall Rules
# -----------------------------------------------------------------------------
echo "--- Firewall Rules ---"
if sudo ufw status | grep -q "22.*ALLOW"; then
    echo "✓ SSH (22) allowed"
    ((PASS++))
else
    echo "✗ SSH (22) - NOT CONFIGURED"
    ((FAIL++))
fi

if sudo ufw status | grep -q "80.*ALLOW"; then
    echo "✓ HTTP (80) allowed"
    ((PASS++))
else
    echo "✗ HTTP (80) - NOT CONFIGURED"
    ((FAIL++))
fi

if sudo ufw status | grep -q "443.*ALLOW"; then
    echo "✓ HTTPS (443) allowed"
    ((PASS++))
else
    echo "✗ HTTPS (443) - NOT CONFIGURED"
    ((FAIL++))
fi
echo ""

# -----------------------------------------------------------------------------
# Project Directory
# -----------------------------------------------------------------------------
echo "--- Project Setup ---"
if [ -d "/home/ubuntu/projects/Claude-Agents" ]; then
    echo "✓ Claude-Agents repo cloned"
    ((PASS++))
else
    echo "✗ Claude-Agents repo - NOT FOUND"
    echo "  (Expected at /home/ubuntu/projects/Claude-Agents)"
    ((FAIL++))
fi

if [ -d "/home/ubuntu/.claude" ]; then
    echo "✓ Claude config directory exists"
    ((PASS++))
else
    echo "✗ Claude config directory - NOT FOUND"
    ((FAIL++))
fi
echo ""

# -----------------------------------------------------------------------------
# Summary
# -----------------------------------------------------------------------------
echo "=============================================="
echo "  VERIFICATION COMPLETE"
echo "=============================================="
echo ""
echo "  Passed: $PASS"
echo "  Failed: $FAIL"
echo ""

if [ $FAIL -eq 0 ]; then
    echo "  ✓ All checks passed!"
    echo ""
    echo "  Next steps:"
    echo "  1. Configure API key: export ANTHROPIC_API_KEY='sk-ant-...'"
    echo "  2. Configure Nginx: sudo nano /etc/nginx/sites-available/claude-agents"
    echo "  3. Set up SSL: sudo certbot --nginx -d yourdomain.com"
else
    echo "  ⚠ Some checks failed. Review the output above."
    echo ""
    echo "  Common fixes:"
    echo "  - Re-run server_setup.sh if components are missing"
    echo "  - Log out and back in for Rust to be in PATH"
    echo "  - Check service status: sudo systemctl status <service>"
fi
echo ""
