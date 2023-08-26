#[cfg(target_family = "wasm")]
mod wasm;
#[cfg(target_family = "wasm")]
pub use wasm::*;

#[cfg(not(target_family = "wasm"))]
mod stub;
#[cfg(not(target_family = "wasm"))]
pub use stub::*;
