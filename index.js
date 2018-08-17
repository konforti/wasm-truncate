const truncate = async (elementId, lines) => {
    const module = await import('./wasm_truncate');
    module.run(elementId, lines);
};
truncate('long-text', 1);
