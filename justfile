deploy:
    dfx stop
    dfx start --background 
    dfx deploy

clean:
    dfx stop
    dfx start --background --clean

redeploy: clean
    dfx deploy

prepare:
    rustup target add wasm32-unknown-unknown
    cargo install candid-extractor

gen:
    cargo build --release --target wasm32-unknown-unknown
    candid-extractor target/wasm32-unknown-unknown/release/certify_v1_backend.wasm > src/certify_v1_backend/certify_v1_backend.did
