# QR Code Decoder

## About

A web application built with Rust and WebAssembly that decodes QR codes. Supports drag-and-drop, file upload, and clipboard paste (Ctrl+V).

## Usage

### Build

Build for production:

```
just build
```

Development build:

```
just dev
```

### Serve locally

```
python3 -m http.server 8000 -d dist
```

Opens at http://localhost:8000

## Tech Stack

- [Yew](https://yew.rs/) - Rust framework for building web apps with WebAssembly
- [rqrr](https://github.com/WanzenBug/rqrr) - Pure Rust QR code decoder
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - Rust/JavaScript interop
