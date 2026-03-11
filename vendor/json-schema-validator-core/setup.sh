#!/bin/bash

# JSON Schema Validator Core - Deployment Setup Script
# This script handles the complete deployment process for the json-schema-validator-core library

set -e

echo "🚀 Starting json-schema-validator-core deployment setup..."

# 1. Initialize Git Repository
echo "📁 Initializing Git repository..."
git init
git add .
git commit -m "Initial release of json-schema-validator-core v1.0.0

- Lightning-fast JSON Schema validation with comprehensive Draft 7 support
- Detailed error reporting with instance paths and custom messages
- High-performance validation engine optimized for throughput
- C FFI exports for multi-language bindings
- WebAssembly bindings for browser and Node.js support
- Built-in format validation (email, URI, date, datetime, IPv4, IPv6, UUID)
- Custom format and keyword extensibility
- Memory-safe implementation with Rust's safety guarantees
- Comprehensive test suite covering all validation scenarios
- Production-ready with extensive documentation"

# 2. Connect to GitHub Repository
echo "🔗 Connecting to GitHub repository..."
git remote add origin https://github.com/rust-core-libs/json-schema-validator-core.git
git branch -M main

# 3. Push to GitHub
echo "⬆️ Pushing to GitHub..."
git push -u origin main

# 4. Run Final Tests
echo "🧪 Running comprehensive test suite..."
cargo test --release

# 5. Build Release Version
echo "🔨 Building optimized release version..."
cargo build --release

# 6. Build WebAssembly Package
echo "🌐 Building WebAssembly package..."
if command -v wasm-pack &> /dev/null; then
    wasm-pack build --target web --out-dir pkg
    echo "✅ WebAssembly package built successfully"
else
    echo "⚠️ wasm-pack not found. Install with: cargo install wasm-pack"
fi

# 7. Run Benchmarks
echo "⚡ Running performance benchmarks..."
if cargo bench --help &> /dev/null; then
    cargo bench
    echo "✅ Benchmarks completed"
else
    echo "⚠️ Benchmark dependencies not available"
fi

# 8. Generate Documentation
echo "📚 Generating documentation..."
cargo doc --no-deps --release

# 9. Publish to Crates.io (commented out for safety)
echo "📦 Ready to publish to Crates.io..."
echo "Run the following command when ready:"
echo "cargo publish"

# 10. Create GitHub Release
echo "🏷️ Creating GitHub release..."
echo "Run the following command to create a release:"
echo "gh release create v1.0.0 --title 'v1.0.0 - Lightning Fast JSON Schema Validator' --notes 'Initial release with comprehensive JSON Schema Draft 7 support, high-performance validation, and multi-language bindings'"

# 11. Package for Distribution
echo "📦 Creating distribution packages..."
cargo package

# 12. Next Steps
echo ""
echo "✅ Setup complete! Next steps:"
echo "1. Review the repository at: https://github.com/rust-core-libs/json-schema-validator-core"
echo "2. Publish to Crates.io: cargo publish"
echo "3. Create GitHub release: gh release create v1.0.0"
echo "4. Set up CI/CD pipeline for automated testing and releases"
echo "5. Create language-specific wrapper libraries (Python, Go, Java, etc.)"
echo "6. Publish WebAssembly package to npm"
echo "7. Add performance benchmarks to CI"
echo "8. Create comprehensive examples and tutorials"

echo ""
echo "📊 Library Stats:"
echo "- Lines of code: $(find src -name '*.rs' -exec wc -l {} + | tail -1 | awk '{print $1}')"
echo "- Test coverage: Run 'cargo tarpaulin' for detailed coverage"
echo "- Binary size (release): $(ls -lh target/release/deps/libjon_schema_validator_core*.rlib 2>/dev/null | awk '{print $5}' || echo 'Not built')"

echo ""
echo "🎉 json-schema-validator-core is ready for the world!"
echo "💡 A lightning-fast JSON Schema validator built with Rust for maximum performance and safety!"
