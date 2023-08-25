![Crates.io](https://img.shields.io/crates/v/wasm-init)

Let's pretend that life-before-main exists for Rust targeting WebAssembly.

# Usage
Add a dependency on `wasm-init`. This crate intentionally provides a no-op implementation for non-wasm platforms, so `cfg` handling isn't necessary and we don't pull in unused extra deps.

```toml
[dependencies]
wasm-init = "0.1"
```

Add as many calls to `wasm_init!` as required on the Rust side, e.g. for decentralised plugin registration or collecting data from all types that use a particular derive macro.

```rust
wasm_init::wasm_init! {
	// literally any code you want to run at startup goes here
}
```

Then, when setting up your web build, make sure you call `wasm_init` from JavaScript/TypeScript.

```html
<script type="module">
	import init, { wasm_init } from "./pkg/my_wasm_crate.js";
	init().then(() => {
		wasm_init();
		// now do things as normal!
	});
</script>
```

# Why?
[inventory](https://github.com/dtolnay/inventory) et al are cool, but can't yet be used when targeting `wasm32-unknown-unknown` or similar.
