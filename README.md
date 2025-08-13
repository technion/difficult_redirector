# Difficult Redirector

This is an Evilginx redirector that uses WebAssembly. There are a few reasons for this:

- WebAssembly is far more difficult to reverse engineer than Javascript
- Google's AI right now claims that Google doesn't execute WebAssembly (some websites disagree). Presumably many bots follow.
- We can hammer the CPU for a delay, hopefully causing some bots to timeout

App boilerplate is based on the following: https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm

# Use

```
wasm-pack build --target web
```

Copy the following files into a redirector folder:
- difficult_redirect_bg.wasm
- difficult_redirect_bg.js
- index.html
