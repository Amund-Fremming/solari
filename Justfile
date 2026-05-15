set shell := ["bash", "-cu"]

# Run the web test app (Next.js)
web:
  cd examples/next-test && npm install && npm run dev

# Run the mobile test app (Expo)
mobile:
  cd examples/expo-test && npm install && npx expo start 

# Run the Axum test server
axum:
  cargo run -p axum-test

# Run the Axum test server with ngrok tunnel
axum-ngrok:
  cd examples/axum-test && NGROK_ENABLED=true cargo run

# Run the Vipps access token example (reads .env)
vipps-examples:
  cargo run -p solari-core --example vipps_scenarios

# Local CI checks for Rust workspace quality gates
local-ci:
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace --all-features
