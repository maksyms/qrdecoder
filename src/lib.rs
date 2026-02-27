#![recursion_limit = "512"]

pub mod app;

use wasm_bindgen::prelude::*;

use app::QrDecoder;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("app").unwrap();
    yew::Renderer::<QrDecoder>::with_root(root).render();
    Ok(())
}
