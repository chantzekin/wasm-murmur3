# `wasm-murmur3`

- 🌟 A WebAssembly implementation of the fast, non-cryptographic hash [murmur3](https://en.wikipedia.org/wiki/MurmurHash)
- 🔋 Use rust crate [murmur3](https://docs.rs/murmur3/0.5.1/murmur3/)
- 📦 Build with [wasm-pack](https://github.com/rustwasm/wasm-pack)


## 🚴 Usage

### 🐑 Use `npm` or `yarn` to install package

```bash
npm install wasm-murmur3 --save
# Or use yarn
yarn add wasm-murmur3
```

### 📖 Import the package on nodejs

```js
const Murmur3 = require('wasm-murmur3')

// hex128: (source: string, seed: number): string
Murmur3.hex128('Hello, world!', 0); // f1512dd1d2d665df2c326650a8f3c564

```

## 🍔 Devlopment

### 🛠️ Build with `wasm-pack build`

```bash
wasm-pack build --target nodejs
# ⬆️ Just build for cjs, you can build for esm ⬇️
wasm-pack build
```

The `pkg` directory is the package directory

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`murmur3`](https://github.com/stusmall/murmur3) this is a rust implementation of the fast, non-cryptographic hash murmur3.
