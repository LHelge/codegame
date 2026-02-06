#!/usr/bin/env bash
# Build all WASM games and copy to frontend/public/wasm
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
GAMES_DIR="$ROOT_DIR/games"
OUTPUT_DIR="$ROOT_DIR/frontend/public/wasm"

# List of games to build (subdirectories in games/)
GAMES=("robotsumo" "snake")

echo "🎮 Building WASM games..."
echo "   Games: ${GAMES[*]}"
echo "   Output: $OUTPUT_DIR"
echo ""

# Ensure wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Install with: cargo install wasm-pack"
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Build each game
for game in "${GAMES[@]}"; do
    echo "🔨 Building $game..."
    
    GAME_DIR="$GAMES_DIR/$game"
    GAME_OUTPUT="$OUTPUT_DIR/$game"
    
    if [ ! -d "$GAME_DIR" ]; then
        echo "❌ Game directory not found: $GAME_DIR"
        exit 1
    fi
    
    # Build with wasm-pack
    wasm-pack build "$GAME_DIR" \
        --target web \
        --out-dir "$GAME_OUTPUT" \
        --out-name "$game"
    
    # Remove unnecessary files
    rm -f "$GAME_OUTPUT/.gitignore"
    rm -f "$GAME_OUTPUT/package.json"
    rm -f "$GAME_OUTPUT/README.md"
    
    echo "✅ $game built successfully"
    echo ""
done

echo "🎉 All games built successfully!"
echo "   Output files:"
for game in "${GAMES[@]}"; do
    echo "   - $OUTPUT_DIR/$game/"
done
