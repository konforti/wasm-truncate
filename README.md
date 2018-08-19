# wasm truncate

> truncate texts by number of lines using [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen).

## Installation

```bash
    npm i -S @konforti/wasm-truncate
```

## Usage

```js
const truncate = async (elementId: string, lines: number) => {
    const module = await import('@konforti/wasm-truncate');
    module.run(elementId, lines);
};

truncate('long-text', 1);
```

## Development

```bash
# Once
rustup target add wasm32-unknown-unknown
cargo +nightly install wasm-bindgen-cli
```

```bash
# Run example
npm start
```
