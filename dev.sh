#!/bin/bash

# Development helper script for Bevy FPS Starter

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Commands
case "$1" in
    "check")
        print_status "Checking code..."
        cargo check
        print_success "Check completed!"
        ;;
    "build")
        print_status "Building project..."
        cargo build
        print_success "Build completed!"
        ;;
    "run")
        print_status "Running FPS Starter..."
        cargo run
        ;;
    "release")
        print_status "Building release version..."
        cargo build --release
        print_success "Release build completed!"
        ;;
    "test")
        print_status "Running tests..."
        cargo test
        print_success "Tests completed!"
        ;;
    "clean")
        print_status "Cleaning build artifacts..."
        cargo clean
        print_success "Clean completed!"
        ;;
    "fmt")
        print_status "Formatting code..."
        cargo fmt
        print_success "Code formatted!"
        ;;
    "clippy")
        print_status "Running clippy..."
        cargo clippy -- -D warnings
        print_success "Clippy completed!"
        ;;
    "doc")
        print_status "Building documentation..."
        cargo doc --open
        ;;
    "setup")
        print_status "Setting up development environment..."
        rustup component add rustfmt clippy
        print_success "Development environment ready!"
        ;;
    "all")
        print_status "Running full development cycle..."
        cargo fmt
        cargo clippy -- -D warnings
        cargo check
        cargo test
        cargo build
        print_success "All checks passed!"
        ;;
    *)
        echo "Bevy FPS Starter - Development Helper"
        echo ""
        echo "Usage: $0 {command}"
        echo ""
        echo "Commands:"
        echo "  check     - Check code for errors"
        echo "  build     - Build debug version"
        echo "  run       - Run the game"
        echo "  release   - Build release version"
        echo "  test      - Run tests"
        echo "  clean     - Clean build artifacts"
        echo "  fmt       - Format code"
        echo "  clippy    - Run linter"
        echo "  doc       - Build and open documentation"
        echo "  setup     - Setup development environment"
        echo "  all       - Run full development cycle"
        echo ""
        echo "Examples:"
        echo "  $0 run"
        echo "  $0 release"
        echo "  $0 all"
        ;;
esac