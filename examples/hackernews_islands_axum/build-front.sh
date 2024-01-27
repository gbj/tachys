mv .cargo/config.wasm.toml .cargo/config.toml
wasm-pack build --target=web --features=hydrate --release
cd pkg
rm *.br
cp hackernews_islands_axum.js hackernews.unmin.js
cat hackernews.unmin.js | esbuild > hackernews_islands_axum.js
brotli hackernews_islands_axum.js
brotli hackernews_islands_axum_bg.wasm
brotli style.css
cd ..
mv .cargo/config.toml .cargo/config.wasm.toml
