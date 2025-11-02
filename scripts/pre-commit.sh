#!/usr/bin/env bash
# 🔍 LogLine Discovery Lab - Pre-commit Hook
# Runs checks before allowing commits

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🔍 Running pre-commit checks...${NC}"
echo ""

# Change to logline_discovery directory if it exists
if [ -d "logline_discovery" ]; then
    cd logline_discovery
fi

# 1. Check formatting
echo -e "${BLUE}1/4 Checking code formatting...${NC}"
if cargo fmt --all -- --check; then
    echo -e "${GREEN}✓ Code is properly formatted${NC}"
else
    echo -e "${RED}✗ Code formatting issues found${NC}"
    echo -e "${YELLOW}Run: cargo fmt --all${NC}"
    exit 1
fi
echo ""

# 2. Run clippy
echo -e "${BLUE}2/4 Running clippy linter...${NC}"
if cargo clippy --all-features -- -D warnings 2>&1 | grep -q "error:"; then
    echo -e "${RED}✗ Clippy found issues${NC}"
    echo -e "${YELLOW}Fix the issues above before committing${NC}"
    exit 1
else
    echo -e "${GREEN}✓ Clippy passed${NC}"
fi
echo ""

# 3. Run tests
echo -e "${BLUE}3/4 Running tests...${NC}"
if cargo test --all 2>&1 | grep -q "test result:.*FAILED"; then
    echo -e "${RED}✗ Tests failed${NC}"
    echo -e "${YELLOW}Fix failing tests before committing${NC}"
    exit 1
else
    echo -e "${GREEN}✓ All tests passed${NC}"
fi
echo ""

# 4. Check for common issues
echo -e "${BLUE}4/4 Checking for common issues...${NC}"

# Check for TODO/FIXME in staged files
if git diff --cached --name-only | xargs grep -l "TODO\|FIXME" 2>/dev/null; then
    echo -e "${YELLOW}⚠ Found TODO/FIXME comments in staged files${NC}"
    echo -e "${YELLOW}  Consider addressing them before committing${NC}"
fi

# Check for debug prints
if git diff --cached --name-only | grep "\.rs$" | xargs grep -l "println!\|dbg!\|eprintln!" 2>/dev/null; then
    echo -e "${YELLOW}⚠ Found debug print statements${NC}"
    echo -e "${YELLOW}  Consider removing them or using proper logging${NC}"
fi

# Check for .env file
if git diff --cached --name-only | grep -q "^\.env$"; then
    echo -e "${RED}✗ Attempting to commit .env file${NC}"
    echo -e "${RED}  This file may contain secrets and should not be committed${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Common issues check passed${NC}"
echo ""

# Success
echo -e "${GREEN}✅ All pre-commit checks passed!${NC}"
echo -e "${BLUE}Proceeding with commit...${NC}"
echo ""

exit 0
