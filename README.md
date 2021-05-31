# CS410: Rust Programming Course Project

## CuteDogStash
CuteDogStash is going to be an open source single-page application for storing posts of dogs. 
Using [Yew](https://github.com/yewstack/yew) and [Trunk](https://github.com/thedodd/trunk), rust code will be bundled into HTML, JavaScript, and WebAssembly to run in browsers. 
Users will be able to post pictures and descriptions which will persist through local storage. 

## Setup
First, ensure Rust is installed(directions can be found [here](https://www.rust-lang.org/tools/install)). 
After you can install trunk and wasm-bindgen using 
```
cargo install trunk wasm-bindgen-cli
```

The following command will build the site and output a link.
```
trunk serve
```
