# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

QR code decoder web application built with Rust and WebAssembly using the Yew framework. Users can drag-and-drop or upload QR code images to decode their contents.

## Build Commands

```bash
just build              # Production build to dist/
just dev                # Development build to dist/
just clean              # Remove dist/
```

## Rust Commands

```bash
cargo fmt -- --check    # Check formatting
cargo clippy            # Run linter
cargo check             # Type check without building
```

## Architecture

- **Entry flow**: `static/loader.js` is an IIFE that dynamically imports the wasm-bindgen ES module and calls `run_app()` from `src/lib.rs`, which renders the Yew `QrDecoder` component into `<div id="app">`
- **QrDecoder component** (`src/app.rs`): Main component handling file drag-and-drop, file upload, QR decoding with `rqrr`, and result display
- **QR decoding**: Uses `image` crate to load image bytes, converts to grayscale, then `rqrr::PreparedImage` detects and decodes QR grids
- **Build pipeline**: `justfile` runs `wasm-pack --target web` to compile Rust to WASM, copies `static/loader.js` as `dist/qrdecoder.js`
- **Styling**: Inline styles using Dracula theme colors defined as constants in `app.rs`
- **Deployment**: Embedded as a widget on schipka.com via `<script src="/apps/qrdecoder/qrdecoder.js">`

## Key Dependencies

- **yew**: Rust framework for building web apps with WASM
- **rqrr**: Pure Rust QR code decoder (WASM-compatible)
- **image**: Image loading and grayscale conversion (configured for WASM with `default-features = false`)
- **gloo-file**: Browser file APIs for reading uploaded/dropped files
- **wasm-bindgen**: Rust/JS interop
