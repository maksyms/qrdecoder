(async function() {
    const base = document.currentScript.src.replace(/[^/]*$/, '');
    const { default: init, run_app } = await import(base + 'qrdecoder_wasm.js');
    await init();
    run_app();
})();
