#!/bin/bash
# ci-local.sh - Cross-platform CI Test Runner
# Run all CI checks locally before pushing

set -e

# Try to find cargo in common locations  
if ! command -v cargo &> /dev/null; then
    # Try common cargo locations across platforms
    CARGO_PATHS=(
        "$HOME/.cargo/bin/cargo"
        "$HOME/.cargo/bin/cargo.exe" 
        "/c/Users/$(whoami)/.cargo/bin/cargo.exe"
        "/home/$(whoami)/.cargo/bin/cargo"
        "$(pwd)/../.cargo/bin/cargo"
    )
    
    for cargo_path in "${CARGO_PATHS[@]}"; do
        if [[ -x "$cargo_path" ]]; then
            export PATH="$(dirname "$cargo_path"):$PATH"
            echo "✓ Found cargo at: $cargo_path"
            break
        fi
    done
    
    # Final check
    if ! command -v cargo &> /dev/null; then
        echo "❌ cargo not found. Make sure Rust is installed."
        echo ""
        echo "To run CI tests:"
        echo "  • Make sure 'cargo --version' works in your terminal"
        echo "  • Or install Rust from https://rustup.rs/"
        exit 1
    fi
fi

echo "✓ Using cargo: $(command -v cargo)"

echo "🦀 Running Local CI Tests"
echo

run_check() {
    local name="$1"
    local command="$2"
    
    echo "Running: $name"
    echo "Command: $command"
    
    start_time=$(date +%s)
    
    if eval "$command"; then
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo "✓ $name completed in ${duration}s"
        echo
        return 0
    else
        end_time=$(date +%s)
        duration=$((end_time - start_time))
        echo "✗ $name failed after ${duration}s"
        echo "❌ CI checks failed. Fix issues before pushing."
        exit 1
    fi
}

# Check if we're in the right directory
if [[ ! -f "Cargo.toml" ]]; then
    echo "❌ Cargo.toml not found. Are you in the project root?"
    exit 1
fi

# Run all CI checks in order
run_check "Format Check" "cargo fmt --all -- --check"
run_check "Clippy Lint" "cargo clippy --all-targets --all-features -- -D warnings"
run_check "Compile Check" "cargo check --verbose"
run_check "Unit Tests" "cargo test --verbose"
run_check "Doc Tests" "cargo test --doc"
run_check "Documentation" "RUSTDOCFLAGS='-D warnings' cargo doc --no-deps"

echo "🎉 All 6 CI checks passed!"
echo "Ready to push to remote."
