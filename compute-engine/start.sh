export NODE_OPTIONS=--openssl-legacy-provider

cd ~/Projects/wwsm/compute-engine

wasm-pack build --target bundler

cd pkg && npm link

cd ../site && npm link compute-engine

npm run serve