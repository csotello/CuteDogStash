# CS410: Rust Programming Course Project

## CuteDogStash
CuteDogStash is an open source single-page application for storing posts of dogs. 
Using [Yew](https://github.com/yewstack/yew) and [Trunk](https://github.com/thedodd/trunk), rust code will be bundled into HTML, JavaScript, and WebAssembly to run in browsers. 
Users will be able to post pictures and descriptions which will persist through local storage. 

## Setup
First, ensure [Rust](https://www.rust-lang.org/tools/install) is installed. 
After you can install trunk and wasm-bindgen using 
```
cargo install trunk wasm-bindgen-cli
```

The following command will build the site and output a link.
```
trunk serve
```

## How It Went
Overall, the project went well. I didn't have any issues with things not working although I did stray from my original design in some ways. My original design had three vectors with expected relational logic to act as the database but I opted for nested vectors. I learned a lot about working with vectors and structs in rust throughout this project as I worked with data and implemented Yew traits.     
