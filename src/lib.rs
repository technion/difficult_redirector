use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlDocument, Document, Location};

fn set_cookie(name: &str, value: &str) {
    let window = window().expect("no global window exists");
    let document: Document = window.document().expect("no document exists");
    let html_document: HtmlDocument = document.dyn_into::<HtmlDocument>().expect("document is not an HtmlDocument");

    let cookie = format!(
        "{}={}; max-age={}; path=/; samesite=lax",
        name,
        value,
        1 * 24 * 60 * 60
    );
    
    html_document.set_cookie(&cookie).expect("could not set cookie")

}

fn redirect_to_google(loc: &str) {
    let window = window().expect("no global window exists");
    let location: Location = window.location();
    
    location.set_href(loc).expect("could not redirect to the specified location")

}

// Only used for debugging purposes
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let encoded_url: Vec<u8> = hex::decode(name).unwrap();
    let decoded_bytes = encoded_url.into_iter().map(|x: u8| x^b'K').collect();
    let decoded_url = String::from_utf8(decoded_bytes).expect("decoded bytes are not valid UTF-8");
    set_cookie("get", "redirected");
    redirect_to_google(&decoded_url)
}