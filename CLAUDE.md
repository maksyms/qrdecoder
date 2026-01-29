# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

QR code decoder web application built with Rust and WebAssembly using the Yew framework. Users can drag-and-drop or upload QR code images to decode their contents.

## Build Commands

```bash
npm install             # Install dependencies (first time setup)
npm run dev             # Start dev server at http://localhost:8000
npm run build           # Production build to dist/
npm run build:dev       # Development build
npm run test            # Run WASM tests in browsers (Chrome, Firefox, Safari)
```

## Rust Commands

```bash
cargo fmt -- --check    # Check formatting
cargo clippy            # Run linter
cargo check             # Type check without building
```

## Architecture

- **Entry flow**: `bootstrap.js` imports the WASM module and calls `run_app()` from `src/lib.rs`, which renders the Yew `QrDecoder` component
- **QrDecoder component** (`src/app.rs`): Main component handling file drag-and-drop, file upload, QR decoding with `rqrr`, and result display
- **QR decoding**: Uses `image` crate to load image bytes, converts to grayscale, then `rqrr::PreparedImage` detects and decodes QR grids
- **Build pipeline**: Webpack 5 bundles JS/SCSS, wasm-pack compiles Rust to WASM, output goes to `dist/`
- **Styling**: SCSS with Dracula UI theme (`style.scss`)

## Key Dependencies

- **yew**: Rust framework for building web apps with WASM
- **rqrr**: Pure Rust QR code decoder (WASM-compatible)
- **image**: Image loading and grayscale conversion (configured for WASM with `default-features = false`)
- **gloo**: Browser APIs for file handling
- **wasm-bindgen**: Rust/JS interop
- **dracula-ui**: CSS theme framework
