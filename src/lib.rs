use futures::{future, Future};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::future_to_promise;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;
use web_sys::{Request, RequestInit, RequestMode, Response};
use js_sys::*;

struct HtmlToImage {}

impl HtmlToImage {

    fn inline(&self, node: &web_sys::HtmlElement, clone: &web_sys::HtmlElement) -> Promise {
        let children = node.children();
        let clone_children = clone.children();
        let mut inline_chilren_promises: Vec<Promise> = Vec::new();

        let src_url = clone.get_attribute("src").unwrap();

        return Promise::new(&mut |resolve, _reject| {
            self.encode_url_as_content(&src_url);
        });

        // if (src_url) {
        //     let src_content = await! 
        // }

        log(&src_url);

        log(&clone_children.length().to_string());
    }

    fn encode_url_as_content(&self, url: &str) -> Promise {
        let mut opts = RequestInit::new();
        opts.method("GET");
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(
            url,
            &opts,
        )
        .unwrap();

        let window = web_sys::window().unwrap();
        let request_promise = window.fetch_with_request(&request);


        let future = JsFuture::from(request_promise)
            .and_then(|resp_value| {
                // `resp_value` is a `Response` object.
                // assert!(resp_value.is_instance_of::<Response>());
                let resp: Response = resp_value.dyn_into().unwrap();
                resp.blob()
            })
            .and_then(|blob: Promise| {
                // Convert this other `Promise` into a rust `Future`.
                JsFuture::from(blob)
            })
            .and_then(|res| {
                let reader = FileReader::new().unwrap();
                let onloadend = Closure::wrap(Box::new(move || {
                    let p = reader.result().unwrap();
                    p.as_string().unwrap().split(",");
                }) as Box<dyn FnMut()>);


                reader.set_onloadend(Some(JsValue::from(onloadend.as_ref())));


                future::ok(JsValue::from(res))
            });

        return future_to_promise(future);
    } 

    fn get_node_dimensions(&self, node: &web_sys::HtmlElement) -> DomRect{
        return node.get_bounding_client_rect();
    }

    fn asSvg(&self, node: &web_sys::HtmlElement) {
        let rect = self.get_node_dimensions(node);
        let clone = HtmlElement::from(JsValue::from(node.clone_node_with_deep(true).unwrap()));
        self.inline(node, &clone);
    }
}

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

fn copy_css(original_element: &web_sys::HtmlElement, target: &web_sys::HtmlElement) {
    let window = web_sys::window().unwrap();
    let computed_style = window.get_computed_style(original_element);
    target.style().set_css_text(&computed_style.unwrap().unwrap().css_text());
}

fn inline_styles(original_element: &web_sys::HtmlElement, target: &web_sys::HtmlElement) {
    // let document = web_sys::window().unwrap().document().unwrap();
    let original_children = original_element.query_selector_all("*");
    let target_children = target.query_selector_all("*");
    let unwrap_target_children = target_children.unwrap();
    let unwrap_original_children = original_children.unwrap();
    let len = unwrap_original_children.length();

    // let original_arr = Array::from(&original_children.unwrap());
    // let target_children_arr = Array::from(&target_children.unwrap());

    copy_css(original_element, target);

    let mut x = 0;
    {
        if x < len {
            let original_child = HtmlElement::from(JsValue::from(unwrap_original_children.item(x).unwrap()));
            let target_child = HtmlElement::from(JsValue::from(unwrap_target_children.item(x).unwrap()));
            copy_css(&original_child, &target_child);
            x += 1;
        }
    }

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


    let mut html_to_image: HtmlToImage  = HtmlToImage {};

    html_to_image.asSvg(node);


    // log("Take document styless");
    // let rules = get_css_rules(&document);
    // let font_rules = get_font_face_rules(&rules);
    // let font_sources = get_fonts_sources(&font_rules);

    // log(&font_sources.len().to_string());

    // let target_element = HtmlElement::from(JsValue::from(node.clone_node().unwrap()));
    // inline_styles(node, &target_element);

    // let xml_serializer = web_sys::XmlSerializer::new();


    // body.append_child(&_svg);

}
