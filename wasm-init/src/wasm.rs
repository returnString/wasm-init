use js_sys::{Array, Function, JsString, Object};
use std::sync::atomic::{AtomicBool, Ordering};
use wasm_bindgen::prelude::*;

#[doc(hidden)]
pub mod __macrodeps {
	pub use gensym::gensym;
	pub use std::{concat, stringify};
}

static INIT_DONE: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "auto-init")]
#[wasm_bindgen(start)]
fn main() {
	wasm_init();
}

/// This function will call all instances of [`crate::wasm_init!`].
///
/// This function can be called either:
/// - from Rust as part of an entrypoint
/// - from JavaScript/TypeScript by calling the exported `wasm_init` function in your module
/// - automatically, by enabling the "auto-init" feature
///
/// This function is idempotent, or safe to call multiple times;
/// your [`crate::wasm_init!`] calls will only be executed once.
#[wasm_bindgen]
pub fn wasm_init() {
	if INIT_DONE.swap(true, Ordering::Relaxed) {
		return;
	};

	let exports: Object = wasm_bindgen::exports().into();
	let entries = Object::entries(&exports);
	for entry in entries {
		let entry = Array::from(&entry);
		let name: JsString = entry.get(0).into();
		let func: Function = entry.get(1).into();

		if name.starts_with("__wasm_init", 0) {
			func.apply(&JsValue::undefined(), &Array::new())
				.expect("func invocation failed");
		}
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __wasm_init_impl {
	($gensym:ident, $($input:tt)*) => {
		const _: () = {
			#[export_name = $crate::__macrodeps::concat!(
				"__wasm_init",
				$crate::__macrodeps::stringify!($gensym)
			)]
			pub extern "C" fn init() {
				$($input)*
			}
		};
	};
}

/// Register code to be run on start-up.
///
/// Each call to this macro will generate a function that will be called exactly once,
/// after the [wasm_init] function is called for the first time.
///
/// You can register code from as many different crates in your project as you'd like;
/// [wasm_init] only needs to be called once.
///
/// # Examples
/// ```
/// trait Plugin {}
///
/// fn register_plugin(plugin: impl Plugin) {
/// 	// grab data from each plugin and store them in a global somewhere
/// }
///
/// struct Plugin1;
///
/// wasm_init::wasm_init! {
/// 	register_plugin(Plugin1);
/// }
///
/// struct Plugin2;
///
/// wasm_init::wasm_init! {
/// 	register_plugin(Plugin2);
/// }
/// ```
#[macro_export]
macro_rules! wasm_init {
	($($input:tt)*) => {
		$crate::__macrodeps::gensym! { $crate::__wasm_init_impl! { $($input)* } }
	};
}
