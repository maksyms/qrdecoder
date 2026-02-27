build:
    wasm-pack build --target web --release --out-dir dist --out-name qrdecoder_wasm --no-typescript
    cp static/loader.js dist/qrdecoder.js
    cp static/index.html dist/
    rm -f dist/.gitignore dist/package.json dist/LICENSE_* dist/README.md

dev:
    wasm-pack build --target web --dev --out-dir dist --out-name qrdecoder_wasm --no-typescript
    cp static/loader.js dist/qrdecoder.js
    cp static/index.html dist/
    rm -f dist/.gitignore dist/package.json dist/LICENSE_* dist/README.md

clean:
    rm -rf dist
