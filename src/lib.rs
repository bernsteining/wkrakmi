use wasm_bindgen::prelude::*;
mod challenge;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn check_flag(flag: &str) -> Result<(), JsValue> {
    let document = web_sys::window()
        .expect("no global `window` exists")
        .document()
        .expect("should have a document on window");

    let alertbox = document
        .create_element("alertbox")
        .expect("should have an alertbox element");

    match challenge::challenge(flag) {
        true => inject_html(
            r#"<div id="alertbox" class="alert_success">
        <span id="close_alertbox" class="closebtn" onclick="remove_alertbox()">×</span>
        <strong>Good job! :-) You can validate the challenge with this flag!</strong>
        </div>
        "#
            .to_string(),
            document,
            alertbox,
        )?,
        false => inject_html(
            r#"<div id="alertbox" class="alert_fail">
        <span id="close_alertbox" class="closebtn" onclick="remove_alertbox()">×</span>
        <strong>Fail!</strong> Try again :-).
        </div>"#
                .to_string(),
            document,
            alertbox,
        )?,
    };

    Ok(())
}

#[wasm_bindgen]
pub fn inject_html(
    input: String,
    document: web_sys::Document,
    element: web_sys::Element,
) -> Result<(), JsValue> {
    element.set_inner_html(&input);
    document
        .body()
        .expect("document should have a body")
        .append_child(&element)?;

    Ok(())
}
