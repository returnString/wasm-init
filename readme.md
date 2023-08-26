[<img src="https://img.shields.io/crates/v/wasm-init">](https://crates.io/crates/wasm-init)
[<img src="https://img.shields.io/badge/docs.rs-wasm--init-green">](https://docs.rs/wasm-init)

Let's pretend that life-before-main exists for Rust targeting WebAssembly.

# Installation
Add a dependency on `wasm-init`. This crate intentionally provides a no-op implementation for non-wasm platforms, so `cfg` handling isn't necessary and we don't pull in unused extra deps.

```toml
[dependencies]
wasm-init = "0.2"
```

# User Code
Add as many calls to `wasm_init!` as required on the Rust side, e.g. for decentralised plugin registration or collecting data from all types that use a particular derive macro.

```rust
wasm_init::wasm_init! {
	// literally any code you want to run at startup goes here
}
```

# Initialisation
To ensure your `wasm_init!` calls are executed on startup, you have three options. Note that all of these options are idempotent - multiple invocations are perfectly safe (if unnecessary), even in a threaded context.

## Auto-init feature
If you'd like to skip manually initialising `wasm-init`, you can enable the `auto-init` feature as part of the dependency:

```toml
[dependencies]
wasm-init = { version = "0.2", features = [ "auto-init" ] }
```

But please bear in mind that this will prevent you from using a [wasm bindgen start function](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html) elsewhere in your project.

## From Rust
Call the `wasm_init::wasm_init` function inside your Rust entrypoint.

## From JavaScript/TypeScript
Call the `wasm_init` export from your built module.

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
