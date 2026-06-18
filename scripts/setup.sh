#!/usr/bin/env bash
set -e

echo "Entra Least-Privilege Analyzer — Setup"
echo "======================================="
echo ""
echo "Prerequisites:"
echo "  - Rust 1.78+ (https://rustup.rs)"
echo "  - An Entra ID app registration with read-only Graph API permissions"
echo "    See docs/graph_api_setup.md for instructions."
echo ""

if [ ! -f .env ]; then
    cp .env.example .env
    echo "Created .env from .env.example — fill in your credentials."
else
    echo ".env already exists."
fi

echo ""
echo "Build:"
echo "  cargo build --release"
echo ""
echo "Run:"
echo "  ./target/release/elpa analyze"
echo "  ./target/release/elpa pim"
echo "  ./target/release/elpa export --format md --output report.md"
