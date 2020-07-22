# call wasm (comlink)

mkdir src
cd src
cargo generate --git https://github.com/rustwasm/wasm-pack-template --name crate
cd ..

npm install --save-dev webpack webpack-cli webpack-dev-server \
html-webpack-plugin clean-webpack-plugin @wasm-tool/wasm-pack-plugin worker-plugin

npm install --save comlink

"start2": "webpack-dev-server --debug --mode production"
