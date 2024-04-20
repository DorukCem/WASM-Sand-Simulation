# What
Sand simulation which runs entirely on the frontend using Rust and WebAssembly

# How to build the project
After cloning this project
```
  wasm-pack build --target bundler
  cd site
  npm i ../pkg
  npm i -D webpack@5 webpack-cli@5 webpack-dev-server@4 copy-webpack-plugin@11
```
    


# How to use
If you make changes to the rust code you must recompile 
```
  Compile: wasm-pack build --target bundler
  Run: cd site && npm run serve
```