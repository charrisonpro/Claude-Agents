#!/bin/bash
# =============================================================================
# Claude Agents Server Setup Script
# Ubuntu EC2 Instance - Full Development Environment
# =============================================================================
# Run as: chmod +x server_setup.sh && sudo ./server_setup.sh
# =============================================================================

set -e  # Exit on any error

echo "=============================================="
echo "  Claude Agents Server Setup"
echo "  Ubuntu EC2 Full Development Environment"
echo "=============================================="

# -----------------------------------------------------------------------------
# PHASE 1: System Update & Core Packages
# -----------------------------------------------------------------------------
echo ""
echo "[1/10] Updating system packages..."
apt update && apt upgrade -y

echo ""
echo "[2/10] Installing core utilities..."
apt install -y \
    build-essential \
    curl \
    wget \
    git \
    vim \
    nano \
    htop \
    tmux \
    unzip \
    zip \
    jq \
    tree \
    software-properties-common \
    apt-transport-https \
    ca-certificates \
    gnupg \
    lsb-release

# -----------------------------------------------------------------------------
# PHASE 2: Security Setup
# -----------------------------------------------------------------------------
echo ""
echo "[3/10] Configuring security..."

# UFW Firewall
apt install -y ufw
ufw default deny incoming
ufw default allow outgoing
ufw allow ssh
ufw allow http
ufw allow https
ufw --force enable

# Fail2ban for SSH protection
apt install -y fail2ban
cat > /etc/fail2ban/jail.local << 'EOF'
[sshd]
enabled = true
port = ssh
filter = sshd
logpath = /var/log/auth.log
maxretry = 5
bantime = 3600
findtime = 600
EOF
systemctl enable fail2ban
systemctl start fail2ban

# Disable root login via SSH (if not already)
sed -i 's/^PermitRootLogin yes/PermitRootLogin no/' /etc/ssh/sshd_config 2>/dev/null || true
sed -i 's/^PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config 2>/dev/null || true

# Automatic security updates
apt install -y unattended-upgrades
dpkg-reconfigure -plow unattended-upgrades

echo "Security configured: UFW enabled, Fail2ban active, auto-updates enabled"

# -----------------------------------------------------------------------------
# PHASE 3: Web Server (Nginx)
# -----------------------------------------------------------------------------
echo ""
echo "[4/10] Installing Nginx..."
apt install -y nginx
systemctl enable nginx
systemctl start nginx

# -----------------------------------------------------------------------------
# PHASE 4: Node.js (for API proxy)
# -----------------------------------------------------------------------------
echo ""
echo "[5/10] Installing Node.js 20 LTS..."
curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
apt install -y nodejs

# Install PM2 globally for process management
npm install -g pm2

# Verify
echo "Node.js version: $(node --version)"
echo "npm version: $(npm --version)"

# -----------------------------------------------------------------------------
# PHASE 5: Rust (for Claude Code CLI and development)
# -----------------------------------------------------------------------------
echo ""
echo "[6/10] Installing Rust..."
# Install as the regular user, not root
sudo -u ubuntu bash << 'RUST_INSTALL'
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup default stable
rustup component add rustfmt clippy
RUST_INSTALL

echo "Rust installed for user 'ubuntu'"

# -----------------------------------------------------------------------------
# PHASE 6: Python & Data Science Stack
# -----------------------------------------------------------------------------
echo ""
echo "[7/10] Installing Python & Data Science tools..."
apt install -y python3 python3-pip python3-venv python3-dev python3-full

# Create a system-wide virtual environment for data science
# (Ubuntu 24.04+ uses PEP 668, blocking global pip installs)
VENV_PATH="/opt/datascience-venv"
python3 -m venv $VENV_PATH
$VENV_PATH/bin/pip install --upgrade pip
$VENV_PATH/bin/pip install \
    numpy \
    pandas \
    scipy \
    matplotlib \
    seaborn \
    scikit-learn \
    jupyter \
    jupyterlab \
    notebook \
    ipython \
    requests \
    httpx \
    pyyaml \
    python-dotenv

# Create symlinks for easy access
ln -sf $VENV_PATH/bin/jupyter /usr/local/bin/jupyter
ln -sf $VENV_PATH/bin/ipython /usr/local/bin/ipython
ln -sf $VENV_PATH/bin/jupyter-lab /usr/local/bin/jupyter-lab

# Add activation helper to ubuntu user's bashrc
echo "" >> /home/ubuntu/.bashrc
echo "# Data science virtual environment" >> /home/ubuntu/.bashrc
echo "alias ds='source /opt/datascience-venv/bin/activate'" >> /home/ubuntu/.bashrc
echo "# Run 'ds' to activate the data science environment" >> /home/ubuntu/.bashrc

echo "Python version: $(python3 --version)"
echo "Data science venv at: $VENV_PATH"
echo "Activate with: source /opt/datascience-venv/bin/activate (or just 'ds')"

# -----------------------------------------------------------------------------
# PHASE 7: R & RStudio Server
# -----------------------------------------------------------------------------
echo ""
echo "[8/10] Installing R and RStudio Server..."

# Add R repository
wget -qO- https://cloud.r-project.org/bin/linux/ubuntu/marutter_pubkey.asc | gpg --dearmor -o /usr/share/keyrings/r-project.gpg
echo "deb [signed-by=/usr/share/keyrings/r-project.gpg] https://cloud.r-project.org/bin/linux/ubuntu $(lsb_release -cs)-cran40/" > /etc/apt/sources.list.d/r-project.list
apt update
apt install -y r-base r-base-dev

# Install common R packages
R -e "install.packages(c('tidyverse', 'ggplot2', 'dplyr', 'readr', 'lubridate', 'jsonlite', 'httr', 'devtools'), repos='https://cloud.r-project.org')"

# RStudio Server (optional - uncomment if needed)
# wget https://download2.rstudio.org/server/jammy/amd64/rstudio-server-2023.12.1-402-amd64.deb
# apt install -y ./rstudio-server-2023.12.1-402-amd64.deb
# rm rstudio-server-*.deb

echo "R version: $(R --version | head -1)"

# -----------------------------------------------------------------------------
# PHASE 8: Database (PostgreSQL & SQLite)
# -----------------------------------------------------------------------------
echo ""
echo "[9/10] Installing databases..."

# PostgreSQL
apt install -y postgresql postgresql-contrib libpq-dev
systemctl enable postgresql
systemctl start postgresql

# SQLite
apt install -y sqlite3 libsqlite3-dev

# Create a database user for the ubuntu user
sudo -u postgres createuser -s ubuntu 2>/dev/null || echo "User 'ubuntu' may already exist in PostgreSQL"

echo "PostgreSQL and SQLite installed"

# -----------------------------------------------------------------------------
# PHASE 9: Claude Code CLI
# -----------------------------------------------------------------------------
echo ""
echo "[10/10] Installing Claude Code CLI..."

# Install via npm (official method)
npm install -g @anthropic-ai/claude-code

# Create config directory
sudo -u ubuntu mkdir -p /home/ubuntu/.claude

echo "Claude Code CLI installed"
echo ""
echo "IMPORTANT: You need to configure your API key:"
echo "  export ANTHROPIC_API_KEY='sk-ant-api03-your-key-here'"
echo "  Or add to /home/ubuntu/.claude/config.toml"

# -----------------------------------------------------------------------------
# PHASE 10: Project Setup
# -----------------------------------------------------------------------------
echo ""
echo "Setting up project directory..."

# Create project directory
sudo -u ubuntu mkdir -p /home/ubuntu/projects
cd /home/ubuntu/projects

# Clone the repository (as ubuntu user)
sudo -u ubuntu git clone https://github.com/charrisonpro/Claude-Agents.git 2>/dev/null || echo "Repository may already exist"

# Set proper ownership
chown -R ubuntu:ubuntu /home/ubuntu/projects

# -----------------------------------------------------------------------------
# FINAL: Summary & Next Steps
# -----------------------------------------------------------------------------
echo ""
echo "=============================================="
echo "  SETUP COMPLETE!"
echo "=============================================="
echo ""
echo "Installed Components:"
echo "  ✓ Core utilities (git, vim, tmux, htop, etc.)"
echo "  ✓ Security (UFW firewall, Fail2ban, auto-updates)"
echo "  ✓ Nginx web server"
echo "  ✓ Node.js 20 LTS + PM2"
echo "  ✓ Rust (for ubuntu user)"
echo "  ✓ Python 3 + Data Science packages"
echo "  ✓ R + common packages"
echo "  ✓ PostgreSQL + SQLite"
echo "  ✓ Claude Code CLI"
echo ""
echo "=============================================="
echo "  NEXT STEPS"
echo "=============================================="
echo ""
echo "1. Log out and back in (or run: source ~/.bashrc)"
echo ""
echo "2. Configure Claude API key:"
echo "   echo 'export ANTHROPIC_API_KEY=\"sk-ant-api03-...\"' >> ~/.bashrc"
echo "   source ~/.bashrc"
echo ""
echo "3. Configure Nginx for your site:"
echo "   sudo nano /etc/nginx/sites-available/claude-agents"
echo ""
echo "4. Set up SSL with Certbot:"
echo "   sudo apt install certbot python3-certbot-nginx"
echo "   sudo certbot --nginx -d yourdomain.com"
echo ""
echo "5. Start the API proxy:"
echo "   cd ~/projects/Claude-Agents/server"
echo "   npm install"
echo "   pm2 start index.js --name claude-api"
echo ""
echo "=============================================="
