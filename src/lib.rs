pub mod app;

use wasm_bindgen::prelude::*;

use app::QrDecoder;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    wasm_logger::init(wasm_logger::Config::default());
    let document = web_sys::window().unwrap().document().unwrap();
    let root = document.get_element_by_id("app").unwrap();
    yew::Renderer::<QrDecoder>::with_root(root).render();
    Ok(())
}
