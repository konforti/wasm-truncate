#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type HTMLDocument;
    static document: HTMLDocument;
    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;
    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;
    #[wasm_bindgen(method, js_name = getElementById)]
    fn getElementById(this: &HTMLDocument, tagName: &str) -> Element;

    type Element;
    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner_html(this: &Element, html: &str);
    #[wasm_bindgen(method, getter = innerHTML)]
    fn get_inner_html(this: &Element) -> String;
    #[wasm_bindgen(method, js_name = appendChild)]
    fn append_child(this: &Element, other: Element);
    #[wasm_bindgen(method, getter = scrollHeight)]
    fn offsetHeight(this: &Element) -> String;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn getComputedStyle(el: &Element) -> CSSStyleDeclaration;

    type CSSStyleDeclaration;
    #[wasm_bindgen(method, js_name = getPropertyValue)]
    fn getPropertyValue(this: &CSSStyleDeclaration, prop: &str) -> String;
}

#[wasm_bindgen]
pub fn run(element_id: &str, lines: u16) {
    let el = document.getElementById(element_id);
    let text = String::from(el.get_inner_html());
    let el_height = || el.offsetHeight();
    let style = getComputedStyle(&el);
    let line_height = || style.getPropertyValue("line-height");
    let font_size = || style.getPropertyValue("font-size").replace("px", "");
    let text_height = || {
        if line_height() == "normal" {
            font_size()
        } else {
            line_height()
        }
    };
    let len_buffer = 5;
    let mut new_text_len;
    let mut new_text = text.clone();
    let get_text_height = || text_height().parse::<u16>().unwrap() * lines;
    let get_el_height = || el_height().parse::<u16>().unwrap();
    let ellipsis = "...";

    // el.set_inner_html(&ellipsis);
    // if get_text_height() < get_el_height() - len_buffer {
    //     // el.set_inner_html(&new_text);
    //     return log("Invalid ellipsis");
    // };

    loop {
        new_text_len = new_text.len() / 2;
        new_text = new_text[..new_text_len].to_string() + ellipsis;
        log(&new_text);
        el.set_inner_html(&new_text);
        if get_text_height() >= get_el_height() - len_buffer || new_text_len < 1 {
            break;
        };
    }

    loop {
        new_text_len += 1;
        new_text = text[..new_text_len].to_string() + ellipsis;
        log(&new_text);
        el.set_inner_html(&new_text);
        if get_text_height() <= get_el_height() - len_buffer || new_text_len > text.len() {
            break;
        };
    }

    new_text_len -= 2;
    new_text = text[..new_text_len].to_string() + ellipsis;
    log(&new_text);
    el.set_inner_html(&new_text);

    // log(line_h   eight);
    // let j = serde_json::to_string(&elem);
    // // let e = d.offsetHeight();
    // log(j);
    // let val = document.createElement("p");
    // val.set_inner_html(e);
    // document.body().append_child(val);
}
