#!/usr/bin/env bash
# 🚀 LogLine Discovery Lab - Quick Setup Script
# Automated setup for new users

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# Functions
print_step() {
    echo -e "${BLUE}${BOLD}==>${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

check_command() {
    if command -v "$1" &> /dev/null; then
        print_success "$1 is installed"
        return 0
    else
        print_error "$1 is not installed"
        return 1
    fi
}

# Banner
clear
echo -e "${BOLD}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}║  🧬 LogLine Discovery Lab - Quick Setup           ║${NC}"
echo -e "${BOLD}║  Automated installation and configuration         ║${NC}"
echo -e "${BOLD}╚════════════════════════════════════════════════════╝${NC}"
echo ""

# Step 1: Check prerequisites
print_step "Step 1/7: Checking prerequisites"
echo ""

MISSING_DEPS=0

if ! check_command "rustc"; then
    print_warning "Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    MISSING_DEPS=1
fi

if ! check_command "cargo"; then
    print_warning "Cargo should come with Rust installation"
    MISSING_DEPS=1
fi

if ! check_command "psql"; then
    print_warning "Install PostgreSQL: brew install postgresql@15 (macOS) or apt install postgresql (Linux)"
    MISSING_DEPS=1
fi

if ! check_command "redis-cli"; then
    print_warning "Install Redis (optional): brew install redis (macOS) or apt install redis (Linux)"
fi

if ! check_command "ollama"; then
    print_warning "Install Ollama: curl https://ollama.ai/install.sh | sh"
    MISSING_DEPS=1
fi

echo ""

if [ $MISSING_DEPS -eq 1 ]; then
    print_error "Missing required dependencies. Please install them and run this script again."
    exit 1
fi

# Step 2: Check services
print_step "Step 2/7: Checking services"
echo ""

if pg_isready -q 2>/dev/null; then
    print_success "PostgreSQL is running"
else
    print_warning "PostgreSQL is not running"
    echo "Start with: brew services start postgresql@15 (macOS) or sudo systemctl start postgresql (Linux)"
    read -p "Press Enter to continue anyway, or Ctrl+C to exit..."
fi

if redis-cli ping &>/dev/null; then
    print_success "Redis is running"
else
    print_warning "Redis is not running (optional, but recommended)"
    echo "Start with: brew services start redis (macOS) or sudo systemctl start redis (Linux)"
fi

if ollama list &>/dev/null; then
    print_success "Ollama is running"

    # Check for required model
    if ollama list | grep -q "mistral:instruct"; then
        print_success "mistral:instruct model is installed"
    else
        print_warning "mistral:instruct model not found"
        read -p "Download now? (y/n): " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            print_step "Downloading mistral:instruct model..."
            ollama pull mistral:instruct
            print_success "Model downloaded"
        fi
    fi
else
    print_warning "Ollama is not running"
    echo "Start with: ollama serve (in another terminal)"
    read -p "Press Enter to continue anyway, or Ctrl+C to exit..."
fi

echo ""

# Step 3: Create database
print_step "Step 3/7: Setting up database"
echo ""

DB_NAME="logline_discovery"
DB_USER="logline"
DB_PASSWORD="logline_dev_password"

if psql -lqt 2>/dev/null | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
    print_success "Database '$DB_NAME' already exists"
else
    print_step "Creating database '$DB_NAME'..."
    createdb "$DB_NAME" 2>/dev/null || print_warning "Could not create database (might already exist or need sudo)"
fi

echo ""

# Step 4: Setup environment file
print_step "Step 4/7: Creating .env file"
echo ""

if [ -f ".env" ]; then
    print_warning ".env file already exists"
    read -p "Overwrite? (y/n): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        print_success "Keeping existing .env file"
    else
        cp .env.example .env
        print_success "Created new .env file from template"
    fi
else
    if [ -f ".env.example" ]; then
        cp .env.example .env
        print_success "Created .env file from template"
    else
        print_error ".env.example not found"
    fi
fi

# Update .env with local values
if [ -f ".env" ]; then
    print_step "Configuring .env with local settings..."

    # Update database URL
    sed -i.bak "s|DATABASE_URL=.*|DATABASE_URL=postgresql://$DB_USER:$DB_PASSWORD@localhost:5432/$DB_NAME|" .env

    # Generate API token
    if command -v openssl &> /dev/null; then
        API_TOKEN=$(openssl rand -hex 32)
        sed -i.bak "s|API_TOKEN=.*|API_TOKEN=$API_TOKEN|" .env
        print_success "Generated secure API token"
    fi

    rm -f .env.bak
    print_success "Configured .env file"
fi

echo ""

# Step 5: Create necessary directories
print_step "Step 5/7: Creating directories"
echo ""

mkdir -p logs
mkdir -p ledger/spans
mkdir -p manuscripts
mkdir -p data

print_success "Created necessary directories"
echo ""

# Step 6: Build project
print_step "Step 6/7: Building project (this may take 5-10 minutes)"
echo ""

cd logline_discovery

print_step "Running cargo build --release..."
if cargo build --release 2>&1 | tee ../build.log; then
    print_success "Build successful!"
else
    print_error "Build failed. Check build.log for details"
    exit 1
fi

cd ..

echo ""

# Step 7: Test installation
print_step "Step 7/7: Testing installation"
echo ""

if [ -f "logline_discovery/target/release/director" ]; then
    print_success "director binary built successfully"
fi

if [ -f "logline_discovery/target/release/discovery_dashboard" ]; then
    print_success "discovery_dashboard binary built successfully"
fi

if [ -f "logline_discovery/target/release/hiv_discovery_runner" ]; then
    print_success "hiv_discovery_runner binary built successfully"
fi

echo ""

# Success message
echo -e "${GREEN}${BOLD}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}${BOLD}║  ✓ Setup completed successfully!                  ║${NC}"
echo -e "${GREEN}${BOLD}╚════════════════════════════════════════════════════╝${NC}"
echo ""

# Next steps
echo -e "${BLUE}${BOLD}📚 Next steps:${NC}"
echo ""
echo "   1. Run the interactive demo:"
echo "      ${YELLOW}./demo.sh${NC}"
echo ""
echo "   2. Start the director in CLI mode:"
echo "      ${YELLOW}./logline_discovery/target/release/director --mode cli${NC}"
echo ""
echo "   3. Start the dashboard:"
echo "      ${YELLOW}./logline_discovery/target/release/discovery_dashboard --port 3000${NC}"
echo ""
echo "   4. Read the documentation:"
echo "      ${YELLOW}cat GETTING_STARTED.md${NC}"
echo ""
echo -e "${GREEN}🎉 Happy discovering!${NC} 🧬"
echo ""
