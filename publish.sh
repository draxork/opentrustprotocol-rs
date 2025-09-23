#!/bin/bash

# 🚀 OpenTrust Protocol Rust SDK - Publication Script
# Maximum Impact Strategy for crates.io

set -e

echo "🦀 OpenTrust Protocol Rust SDK - Publication Script"
echo "=================================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
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

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the project root."
    exit 1
fi

print_status "Starting publication process for OpenTrust Protocol v0.2.0..."

# Step 1: Pre-publication checks
print_status "Step 1: Running pre-publication checks..."

print_status "Checking Rust toolchain..."
if ! command -v cargo &> /dev/null; then
    print_error "Cargo not found. Please install Rust first."
    exit 1
fi

print_status "Running cargo check..."
if ! cargo check; then
    print_error "Cargo check failed. Please fix compilation errors."
    exit 1
fi
print_success "Code compiles successfully!"

print_status "Running tests..."
if ! cargo test; then
    print_error "Tests failed. Please fix test failures."
    exit 1
fi
print_success "All tests passed!"

print_status "Running clippy..."
if ! cargo clippy -- -D warnings; then
    print_warning "Clippy found issues. Consider fixing them for better code quality."
fi

print_status "Formatting code..."
if ! cargo fmt; then
    print_warning "Code formatting failed."
fi

# Step 2: Documentation check
print_status "Step 2: Checking documentation..."

print_status "Building documentation..."
if ! cargo doc --no-deps; then
    print_error "Documentation build failed."
    exit 1
fi
print_success "Documentation built successfully!"

# Step 3: Example verification
print_status "Step 3: Verifying examples..."

print_status "Running mapper examples..."
if ! cargo run --example mapper_examples; then
    print_error "Example execution failed."
    exit 1
fi
print_success "Examples run successfully!"

# Step 4: Package verification
print_status "Step 4: Verifying package..."

print_status "Checking package metadata..."
PACKAGE_NAME=$(grep '^name = ' Cargo.toml | cut -d'"' -f2)
VERSION=$(grep '^version = ' Cargo.toml | cut -d'"' -f2)

print_success "Package: $PACKAGE_NAME v$VERSION"

# Step 5: Dry run publication
print_status "Step 5: Dry run publication..."

print_status "Running cargo publish --dry-run..."
if ! cargo publish --dry-run; then
    print_error "Dry run failed. Please fix publication issues."
    exit 1
fi
print_success "Dry run successful!"

# Step 6: Final confirmation
echo ""
echo "🎯 PUBLICATION SUMMARY"
echo "====================="
echo "Package: $PACKAGE_NAME"
echo "Version: $VERSION"
echo "Repository: https://github.com/draxork/opentrustprotocol-rs"
echo "Documentation: https://docs.rs/$PACKAGE_NAME"
echo ""
echo "✅ All checks passed!"
echo "✅ Ready for publication!"
echo ""

read -p "🚀 Do you want to publish to crates.io? (y/N): " -n 1 -r
echo ""

if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_status "Publishing to crates.io..."
    
    if cargo publish; then
        print_success "🎉 Successfully published $PACKAGE_NAME v$VERSION to crates.io!"
        echo ""
        echo "📊 NEXT STEPS:"
        echo "============="
        echo "1. Verify publication: https://crates.io/crates/$PACKAGE_NAME"
        echo "2. Check documentation: https://docs.rs/$PACKAGE_NAME"
        echo "3. Post on r/rust (Reddit)"
        echo "4. Submit to Hacker News"
        echo "5. Tweet announcement"
        echo "6. Post on LinkedIn"
        echo ""
        echo "🎯 MARKETING STRATEGY:"
        echo "===================="
        echo "- Use the content from publish_strategy.md"
        echo "- Time posts for maximum engagement"
        echo "- Engage with community responses"
        echo "- Track metrics and adjust strategy"
        echo ""
        print_success "Publication complete! Time to spread the word! 🚀"
    else
        print_error "Publication failed. Please check the error messages above."
        exit 1
    fi
else
    print_warning "Publication cancelled by user."
    echo ""
    echo "💡 To publish later, run: cargo publish"
    echo "📋 Marketing strategy available in: publish_strategy.md"
fi

echo ""
echo "🦀 OpenTrust Protocol - Making Trust Auditable! 🦀"


