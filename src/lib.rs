use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use js_sys::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}


fn get_css_rules(document: &web_sys::Document) -> Vec<web_sys::CssRule> {
    let stylesheets: Array = Array::from(&document.style_sheets());
    let mut document_css_rules = Vec::new();

    stylesheets.for_each(&mut |style, _, _| {
        let css_style_sheet = CssStyleSheet::from(JsValue::from(style));
        let rules = Array::from(&css_style_sheet.css_rules().unwrap());

        rules.for_each(&mut |rule, _, _| {
            document_css_rules.push(web_sys::CssRule::from(rule));
        });
    });

    return document_css_rules;
}

fn get_font_face_rules(rules: &Vec<web_sys::CssRule>) -> Vec<web_sys::CssFontFaceRule> {
    let mut font_rules = Vec::new();

   rules.iter().for_each(|rule| {
       if(rule.type_() == web_sys::CssRule::FONT_FACE_RULE) {
           font_rules.push(CssFontFaceRule::from(JsValue::from(rule)));
       }
   });

    return font_rules;
}

fn get_fonts_sources(font_rules: &Vec<web_sys::CssFontFaceRule>) -> Vec<String> {
    let mut font_sources = Vec::new();

    font_rules.iter().for_each(|rule| {
        font_sources.push(rule.style().get_property_value("src").unwrap());
    });

    return font_sources;
}

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


    log("Take document styless");
    let rules = get_css_rules(&document);
    let font_rules = get_font_face_rules(&rules);
    let font_sources = get_fonts_sources(&font_rules);

    log(&font_sources.len().to_string());


    body.append_child(&_svg);

}
