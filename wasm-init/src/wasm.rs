use js_sys::{Array, Function, JsString, Object};
use wasm_bindgen::JsValue;

#[doc(hidden)]
pub use gensym::gensym;
#[doc(hidden)]
pub use paste;
#[doc(hidden)]
pub use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn wasm_init() {
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
		$crate::paste::item! {
			#[$crate::wasm_bindgen]
			pub fn [<__wasm_init $gensym>]() {
				$($input)*
			}
		}
	};
}

#[macro_export]
macro_rules! wasm_init {
	($($input:tt)*) => {
		$crate::gensym! { $crate::__wasm_init_impl! { $($input)* } }
	};
}
