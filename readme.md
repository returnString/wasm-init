Let's pretend that life-before-main exists for Rust targeting WebAssembly.

# Usage
```rust
wasm_init::wasm_init! {
	// literally any code you want to run goes here
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
