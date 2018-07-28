const truncate = async elementId => {
  const module = await import("./wasm_truncate");
  module.run(elementId);
};
truncate("long-text");
