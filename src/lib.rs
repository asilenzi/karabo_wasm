pub mod karabo_hash;
pub mod binary_writers;
pub mod binary_readers;
mod binary_test;
pub mod web_socket;
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
