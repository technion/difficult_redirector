# Difficult Redirector

This is an Evilginx redirector that uses WebAssembly. There are a few reasons for this:

- WebAssembly is far more difficult to reverse engineer than Javascript
- Google's AI right now claims that Google doesn't execute WebAssembly (some websites disagree). Presumably many bots follow.
- We can hammer the CPU for a delay, hopefully causing some bots to timeout

App boilerplate is based on the following: https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm

There is also code to set a cookie as boilerplate for using WebAssembly to perform traditional JS based bot detection.

# Use

**Build**
```
wasm-pack build --target web
```

**Generate a lure with Evilginx**

As normal

**Setup the Redirector**

Copy the following files into a redirector folder:
- difficult_redirect_bg.wasm
- difficult_redirect_bg.js
- index.html

**Publish a working Redirector**

- Use the redirector. This will display the proper encoded URL, then redirect to the template site
- Use this link to obfuscate the lure URL using our super secret format: [CyberChef](https://cyberchef.org/#recipe=XOR(%7B'option':'UTF8','string':'K'%7D,'Standard',false)To_Hex('None',0))
- Place the encoded url in the index.html file, greet() function

Bonus Points:

- Minify the .js file (don't obfuscate, this is all WASM boilerplate)
- Make a beautiful index.html with a stylish loading banner. This has been left for the reader to avoid any indicators

**Development Guidelines**

- Rust code is built with deny(unsafe_code)
- Clippy tests
- Allegedly panic free
- I tried to write tests but it takes a whole headless browser to run them - not currently worth it

**TODO**

- [x] Introduce builds and CI process
- [x] Better error handling
- [x] Implement CPU time delays

