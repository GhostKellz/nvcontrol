#!/bin/bash
# Quick test script for nvcontrol Docker environment

set -e

cd "$(dirname "$0")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 nvcontrol Docker Test Runner${NC}\n"

# Function to print section headers
section() {
    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${GREEN}$1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"
}

# Check prerequisites
check_prereqs() {
    section "Checking Prerequisites"

    if ! command -v docker &> /dev/null; then
        echo -e "${RED}❌ Docker not found${NC}"
        echo "Install: sudo pacman -S docker"
        exit 1
    fi
    echo -e "${GREEN}✅ Docker installed${NC}"

    if ! command -v docker-compose &> /dev/null; then
        echo -e "${RED}❌ Docker Compose not found${NC}"
        echo "Install: sudo pacman -S docker-compose"
        exit 1
    fi
    echo -e "${GREEN}✅ Docker Compose installed${NC}"

    if ! docker ps &> /dev/null; then
        echo -e "${RED}❌ Docker daemon not accessible${NC}"
        echo "Run: sudo systemctl start docker"
        echo "Or: sudo usermod -aG docker \$USER (then logout/login)"
        exit 1
    fi
    echo -e "${GREEN}✅ Docker daemon running${NC}"

    # Check for NVIDIA GPU
    if command -v nvidia-smi &> /dev/null; then
        echo -e "${GREEN}✅ NVIDIA GPU detected${NC}"
        nvidia-smi --query-gpu=name --format=csv,noheader | while read gpu; do
            echo -e "   ${BLUE}→${NC} $gpu"
        done
    else
        echo -e "${YELLOW}⚠️  nvidia-smi not found (GPU tests will fail)${NC}"
    fi

    # Check for nvidia-container-toolkit
    if docker run --rm --gpus all nvidia/cuda:12.0-base nvidia-smi &> /dev/null; then
        echo -e "${GREEN}✅ NVIDIA Container Toolkit working${NC}"
    else
        echo -e "${YELLOW}⚠️  NVIDIA Container Toolkit not configured${NC}"
        echo "   Install: sudo pacman -S nvidia-container-toolkit"
        echo "   Configure: sudo nvidia-ctk runtime configure --runtime=docker"
        echo "   Restart: sudo systemctl restart docker"
    fi
}

# Show usage
usage() {
    echo "Usage: $0 [OPTION]"
    echo ""
    echo "Options:"
    echo "  all             Run all tests (lint + unit + integration)"
    echo "  quick           Quick tests (lint + unit only)"
    echo "  lint            Run Clippy linter"
    echo "  unit            Run unit tests"
    echo "  integration     Run integration tests (requires GPU)"
    echo "  bench           Run benchmarks"
    echo "  coverage        Generate code coverage"
    echo "  dev             Start interactive development shell"
    echo "  build           Build Docker image"
    echo "  clean           Clean up containers and volumes"
    echo "  prereqs         Check prerequisites"
    echo ""
    echo "Examples:"
    echo "  $0 quick        # Fast feedback (no GPU needed)"
    echo "  $0 all          # Full test suite"
    echo "  $0 dev          # Interactive shell for debugging"
}

# Main commands
case "${1:-help}" in
    prereqs)
        check_prereqs
        ;;

    build)
        section "Building Docker Image"
        docker-compose build
        echo -e "${GREEN}✅ Build complete${NC}"
        ;;

    lint)
        check_prereqs
        section "Running Clippy Linter"
        docker-compose up --exit-code-from nvcontrol-lint nvcontrol-lint
        echo -e "${GREEN}✅ Linting passed${NC}"
        ;;

    unit)
        check_prereqs
        section "Running Unit Tests"
        docker-compose up --exit-code-from nvcontrol-unit-tests nvcontrol-unit-tests
        echo -e "${GREEN}✅ Unit tests passed${NC}"
        ;;

    integration)
        check_prereqs
        section "Running Integration Tests (GPU Required)"
        docker-compose up --exit-code-from nvcontrol-integration-tests nvcontrol-integration-tests
        echo -e "${GREEN}✅ Integration tests passed${NC}"
        ;;

    bench)
        check_prereqs
        section "Running Benchmarks"
        docker-compose up --exit-code-from nvcontrol-bench nvcontrol-bench
        echo -e "${GREEN}✅ Benchmarks complete${NC}"
        echo -e "Results: ${BLUE}../bench-results/${NC}"
        ;;

    coverage)
        check_prereqs
        section "Generating Code Coverage"
        docker-compose up --exit-code-from nvcontrol-coverage nvcontrol-coverage
        echo -e "${GREEN}✅ Coverage report generated${NC}"
        echo -e "Report: ${BLUE}../coverage/index.html${NC}"
        ;;

    quick)
        check_prereqs
        section "Quick Test Suite (Lint + Unit)"

        echo -e "${YELLOW}Running Clippy...${NC}"
        docker-compose up --exit-code-from nvcontrol-lint nvcontrol-lint

        echo -e "${YELLOW}Running unit tests...${NC}"
        docker-compose up --exit-code-from nvcontrol-unit-tests nvcontrol-unit-tests

        echo -e "\n${GREEN}✅ Quick tests passed!${NC}"
        ;;

    all)
        check_prereqs
        section "Full Test Suite"

        echo -e "${YELLOW}1/3 Running Clippy...${NC}"
        docker-compose up --exit-code-from nvcontrol-lint nvcontrol-lint

        echo -e "${YELLOW}2/3 Running unit tests...${NC}"
        docker-compose up --exit-code-from nvcontrol-unit-tests nvcontrol-unit-tests

        echo -e "${YELLOW}3/3 Running integration tests...${NC}"
        docker-compose up --exit-code-from nvcontrol-integration-tests nvcontrol-integration-tests

        echo -e "\n${GREEN}✅ All tests passed!${NC}"
        ;;

    dev)
        check_prereqs
        section "Interactive Development Shell"
        echo -e "${BLUE}Starting container with GPU access...${NC}"
        echo -e "${YELLOW}Tip: Run 'nvctl doctor' to verify NVIDIA setup${NC}\n"
        docker-compose run --rm nvcontrol-dev
        ;;

    clean)
        section "Cleaning Up"
        echo -e "${YELLOW}Stopping containers...${NC}"
        docker-compose down

        read -p "Remove volumes (including Cargo cache)? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            docker-compose down -v
            echo -e "${GREEN}✅ Removed containers and volumes${NC}"
        else
            echo -e "${GREEN}✅ Removed containers (kept volumes)${NC}"
        fi

        read -p "Remove Docker image? [y/N] " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            docker rmi nvcontrol:test
            echo -e "${GREEN}✅ Removed image${NC}"
        fi
        ;;

    help|--help|-h)
        usage
        ;;

    *)
        echo -e "${RED}Unknown option: $1${NC}\n"
        usage
        exit 1
        ;;
esac
