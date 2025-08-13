use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlDocument, Document, Location};

#[wasm_bindgen]
pub fn set_cookie(name: &str, value: &str) {
    let window = window().expect("no global window exists");
    let document: Document = window.document().expect("no document exists");
    let html_document: HtmlDocument = document.dyn_into::<HtmlDocument>().expect("document is not an HtmlDocument");

    let cookie = format!(
        "{}={}; max-age={}; path=/; samesite=lax",
        name,
        value,
        1 * 24 * 60 * 60
    );
    
    html_document.set_cookie(&cookie).expect("could not set cookie");

}

#[wasm_bindgen]
pub fn redirect_to_google(loc: &str) {
    let window = window().expect("no global window exists");
    let location: Location = window.location();
    
    location.set_href(loc).expect("could not redirect to the specified location");

}

#[wasm_bindgen]
pub fn greet(name: &str) {
    set_cookie("get", "redirected");
    redirect_to_google(name);
}