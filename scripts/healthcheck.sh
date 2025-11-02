#!/usr/bin/env bash
# 🏥 LogLine Discovery Lab - Health Check Script
# Verifies all services and components are running correctly

set -euo pipefail

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

# Counters
PASSED=0
FAILED=0
WARNINGS=0

# Functions
check_pass() {
    echo -e "${GREEN}✓${NC} $1"
    ((PASSED++))
}

check_fail() {
    echo -e "${RED}✗${NC} $1"
    ((FAILED++))
}

check_warn() {
    echo -e "${YELLOW}⚠${NC} $1"
    ((WARNINGS++))
}

print_header() {
    echo -e "${BLUE}${BOLD}$1${NC}"
}

# Banner
clear
echo -e "${BOLD}╔════════════════════════════════════════════════════╗${NC}"
echo -e "${BOLD}║  🏥 LogLine Discovery Lab - Health Check          ║${NC}"
echo -e "${BOLD}║  Verifying system components                      ║${NC}"
echo -e "${BOLD}╚════════════════════════════════════════════════════╝${NC}"
echo ""

# 1. Check Rust installation
print_header "1. Checking Rust Installation"
if command -v rustc &> /dev/null; then
    VERSION=$(rustc --version | cut -d' ' -f2)
    if [[ "$(echo -e "1.70\n$VERSION" | sort -V | head -n1)" == "1.70" ]]; then
        check_pass "Rust $VERSION (>= 1.70 required)"
    else
        check_fail "Rust $VERSION (< 1.70, upgrade required)"
    fi
else
    check_fail "Rust not installed"
fi

if command -v cargo &> /dev/null; then
    check_pass "Cargo $(cargo --version | cut -d' ' -f2)"
else
    check_fail "Cargo not installed"
fi
echo ""

# 2. Check PostgreSQL
print_header "2. Checking PostgreSQL"
if command -v psql &> /dev/null; then
    check_pass "PostgreSQL client installed"

    if pg_isready -q 2>/dev/null; then
        check_pass "PostgreSQL server is running"

        # Check specific database
        if psql -lqt 2>/dev/null | cut -d \| -f 1 | grep -qw "logline_discovery"; then
            check_pass "Database 'logline_discovery' exists"
        else
            check_warn "Database 'logline_discovery' not found"
        fi
    else
        check_fail "PostgreSQL server is not running"
    fi
else
    check_fail "PostgreSQL not installed"
fi
echo ""

# 3. Check Redis
print_header "3. Checking Redis (Optional)"
if command -v redis-cli &> /dev/null; then
    check_pass "Redis client installed"

    if redis-cli ping &>/dev/null; then
        REDIS_VERSION=$(redis-cli --version | grep -oP '\d+\.\d+\.\d+' | head -1)
        check_pass "Redis server running (v$REDIS_VERSION)"
    else
        check_warn "Redis server not running (optional, but recommended)"
    fi
else
    check_warn "Redis not installed (optional, but recommended)"
fi
echo ""

# 4. Check Ollama
print_header "4. Checking Ollama"
if command -v ollama &> /dev/null; then
    check_pass "Ollama CLI installed"

    if ollama list &>/dev/null; then
        check_pass "Ollama server is running"

        # Check for required model
        if ollama list | grep -q "mistral:instruct"; then
            check_pass "Model 'mistral:instruct' is available"
        else
            check_warn "Model 'mistral:instruct' not found (run: ollama pull mistral:instruct)"
        fi
    else
        check_fail "Ollama server is not running"
    fi
else
    check_fail "Ollama not installed"
fi
echo ""

# 5. Check environment configuration
print_header "5. Checking Configuration"
if [ -f ".env" ]; then
    check_pass ".env file exists"

    # Check critical variables
    if grep -q "DATABASE_URL=" .env && ! grep -q "DATABASE_URL=postgresql://logline:your_password_here" .env; then
        check_pass "DATABASE_URL configured"
    else
        check_warn "DATABASE_URL not configured or using default"
    fi

    if grep -q "OLLAMA_URL=" .env; then
        check_pass "OLLAMA_URL configured"
    else
        check_warn "OLLAMA_URL not configured"
    fi
else
    check_fail ".env file not found (copy from .env.example)"
fi

if [ -f ".env.example" ]; then
    check_pass ".env.example exists"
else
    check_warn ".env.example not found"
fi
echo ""

# 6. Check directories
print_header "6. Checking Directories"
for dir in logs ledger/spans manuscripts data; do
    if [ -d "$dir" ]; then
        check_pass "Directory '$dir' exists"
    else
        check_warn "Directory '$dir' not found (will be created on first run)"
    fi
done
echo ""

# 7. Check binaries
print_header "7. Checking Binaries"
BINARIES=("director" "discovery_dashboard" "hiv_discovery_runner" "job_scheduler" "job_worker" "job_client")

for binary in "${BINARIES[@]}"; do
    BINARY_PATH="logline_discovery/target/release/$binary"
    if [ -f "$BINARY_PATH" ]; then
        SIZE=$(du -h "$BINARY_PATH" | cut -f1)
        check_pass "$binary ($SIZE)"
    else
        check_warn "$binary not built (run: cargo build --release -p $binary)"
    fi
done
echo ""

# 8. Check API endpoints (if running)
print_header "8. Checking API Endpoints (if running)"
if curl -s http://localhost:8080/health &>/dev/null; then
    check_pass "Director API responding on port 8080"
else
    check_warn "Director API not running on port 8080 (expected if not started)"
fi

if curl -s http://localhost:3000 &>/dev/null; then
    check_pass "Dashboard responding on port 3000"
else
    check_warn "Dashboard not running on port 3000 (expected if not started)"
fi
echo ""

# 9. Check disk space
print_header "9. Checking System Resources"
DISK_USAGE=$(df -h . | awk 'NR==2 {print $5}' | sed 's/%//')
if [ "$DISK_USAGE" -lt 80 ]; then
    check_pass "Disk space: ${DISK_USAGE}% used (sufficient)"
elif [ "$DISK_USAGE" -lt 90 ]; then
    check_warn "Disk space: ${DISK_USAGE}% used (low space)"
else
    check_fail "Disk space: ${DISK_USAGE}% used (critical)"
fi

# Check memory
if command -v free &> /dev/null; then
    MEM_USAGE=$(free | awk 'NR==2 {printf "%.0f", $3/$2 * 100}')
    if [ "$MEM_USAGE" -lt 80 ]; then
        check_pass "Memory: ${MEM_USAGE}% used"
    else
        check_warn "Memory: ${MEM_USAGE}% used (high usage)"
    fi
fi
echo ""

# 10. Quick connectivity test
print_header "10. Network Connectivity"
if ping -c 1 8.8.8.8 &>/dev/null; then
    check_pass "Internet connectivity"
else
    check_warn "No internet connectivity (may affect model downloads)"
fi

if ping -c 1 crates.io &>/dev/null; then
    check_pass "crates.io reachable"
else
    check_warn "crates.io not reachable (may affect cargo builds)"
fi
echo ""

# Summary
echo -e "${BOLD}════════════════════════════════════════════════════${NC}"
echo -e "${BOLD}Summary${NC}"
echo -e "${BOLD}════════════════════════════════════════════════════${NC}"
echo ""
echo -e "  ${GREEN}Passed:${NC}   $PASSED"
echo -e "  ${YELLOW}Warnings:${NC} $WARNINGS"
echo -e "  ${RED}Failed:${NC}   $FAILED"
echo ""

# Final verdict
if [ "$FAILED" -eq 0 ] && [ "$WARNINGS" -eq 0 ]; then
    echo -e "${GREEN}${BOLD}✓ All checks passed! System is healthy.${NC}"
    EXIT_CODE=0
elif [ "$FAILED" -eq 0 ]; then
    echo -e "${YELLOW}${BOLD}⚠ System is operational with warnings.${NC}"
    echo -e "${YELLOW}  Review warnings above for optional improvements.${NC}"
    EXIT_CODE=0
else
    echo -e "${RED}${BOLD}✗ System has critical issues.${NC}"
    echo -e "${RED}  Fix failed checks before proceeding.${NC}"
    EXIT_CODE=1
fi

echo ""

# Recommendations
if [ "$FAILED" -gt 0 ] || [ "$WARNINGS" -gt 0 ]; then
    echo -e "${BLUE}${BOLD}Recommendations:${NC}"
    echo ""

    if ! command -v rustc &> /dev/null; then
        echo "  • Install Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi

    if ! pg_isready -q 2>/dev/null; then
        echo "  • Start PostgreSQL: brew services start postgresql@15 (macOS)"
        echo "                      sudo systemctl start postgresql (Linux)"
    fi

    if ! redis-cli ping &>/dev/null; then
        echo "  • Start Redis: brew services start redis (macOS)"
        echo "                 sudo systemctl start redis (Linux)"
    fi

    if ! ollama list &>/dev/null; then
        echo "  • Start Ollama: ollama serve"
    fi

    if ! ollama list | grep -q "mistral:instruct"; then
        echo "  • Download model: ollama pull mistral:instruct"
    fi

    if [ ! -f ".env" ]; then
        echo "  • Create .env: cp .env.example .env"
    fi

    if [ ! -f "logline_discovery/target/release/director" ]; then
        echo "  • Build project: cd logline_discovery && cargo build --release"
    fi

    echo ""
fi

exit $EXIT_CODE
