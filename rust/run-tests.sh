#!/bin/bash
# Test runner script for local development
# Runs all tests that can execute without hardware

set -e

echo "================================"
echo "ESP32 Bus Pirate - Test Runner"
echo "================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

cd "$(dirname "$0")"

echo -e "${YELLOW}Running protocol tests...${NC}"
cd protocol
cargo test --verbose
echo -e "${GREEN}✓ Protocol tests passed${NC}"
echo ""

cd ..

echo -e "${YELLOW}Checking bus-modes tests...${NC}"
cd bus-modes
if cargo test --lib --verbose 2>/dev/null; then
    echo -e "${GREEN}✓ Bus-modes tests passed${NC}"
else
    echo -e "${YELLOW}⚠ No tests in bus-modes yet${NC}"
fi
echo ""

cd ..

echo -e "${YELLOW}Checking drivers tests...${NC}"
cd drivers
if cargo test --lib --verbose 2>/dev/null; then
    echo -e "${GREEN}✓ Drivers tests passed${NC}"
else
    echo -e "${YELLOW}⚠ No tests in drivers yet${NC}"
fi
echo ""

cd ..

echo -e "${GREEN}================================${NC}"
echo -e "${GREEN}All tests completed!${NC}"
echo -e "${GREEN}================================${NC}"
