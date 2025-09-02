#!/bin/bash

# Verification script for world-simulation installation

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${YELLOW}World Simulation Installation Verification${NC}"
echo "=========================================="
echo

# Check Docker
echo -n "Checking Docker... "
if command -v docker &> /dev/null; then
    echo -e "${GREEN}✓ Installed${NC} ($(docker --version))"
else
    echo -e "${RED}✗ Not found${NC}"
fi

# Check Docker Compose
echo -n "Checking Docker Compose... "
if command -v docker-compose &> /dev/null; then
    echo -e "${GREEN}✓ Installed${NC} ($(docker-compose --version))"
else
    echo -e "${RED}✗ Not found${NC}"
fi

# Check project files
echo
echo "Checking project files:"
files=(
    "Dockerfile"
    "docker-compose.yml"
    "start-simulation.sh"
    "world-simulation/Cargo.toml"
    "world-simulation/spacetime.toml"
    "world-simulation/src/lib.rs"
)

for file in "${files[@]}"; do
    echo -n "  $file... "
    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC}"
    else
        echo -e "${RED}✗${NC}"
    fi
done

echo
echo -e "${YELLOW}Ready to start the simulation!${NC}"
echo "Run: ./start-simulation.sh start"