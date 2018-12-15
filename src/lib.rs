use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn draw(node: &web_sys::HtmlElement) {
    let document = web_sys::window().unwrap().document().unwrap();
    let _svg = document.create_element("svg").unwrap();
    let body = document.body().expect("document should have a body");

    let xmlns: Option<&str> = Some("xmlns");

    _svg.set_attribute("xmlns", "http://www.w3.org/2000/svg");
    _svg.set_attribute_ns(xmlns, "xlink", "http://www.w3.org/1999/xlink");

    let width = node.client_width().to_string();
    let height = node.client_height().to_string();
    let viewbox = format!("0 0 {} {}", width, height);

    _svg.set_attribute("width", &width);
    _svg.set_attribute("height", &height);
    _svg.set_attribute("viewBox",  &viewbox);

    body.append_child(&_svg);

}
