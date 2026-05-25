# Configuration

## Backend (Axum)

```env
AXUM_PORT=3001
```

## Vipps Backend Credentials

```env
VIPPS_BASE_URL=https://apitest.vipps.no
VIPPS_CLIENT_ID=your_vipps_client_id
VIPPS_CLIENT_SECRET=your_vipps_client_secret
VIPPS_SUBSCRIPTION_KEY=your_vipps_subscription_key
VIPPS_MSN=your_vipps_msn
```

## Stripe Backend Credentials

```env
STRIPE_API_BASE_URL=https://api.stripe.com
STRIPE_SECRET_KEY=your_stripe_secret_key
STRIPE_PUBLISHABLE_KEY=your_stripe_publishable_key
STRIPE_WEBHOOK_SECRET=your_stripe_webhook_secret
```

## Optional ngrok Auto-Tunnel

```env
NGROK_ENABLED=true
NGROK_DOMAIN=https://setigerous-tamela-agitable.ngrok-free.dev
NGROK_AUTHTOKEN=your_ngrok_authtoken
```

## Frontend (Expo)

```env
EXPO_PUBLIC_AXUM_BASE_URL=https://setigerous-tamela-agitable.ngrok-free.dev
EXPO_PUBLIC_STRIPE_PUBLISHABLE_KEY=${STRIPE_PUBLISHABLE_KEY}
EXPO_PUBLIC_STRIPE_MERCHANT_IDENTIFIER=merchant.com.solari.test
```

## Frontend (Next.js)

```env
NEXT_PUBLIC_AXUM_BASE_URL=http://127.0.0.1:3001
NEXT_PUBLIC_VIPPS_WEB_RETURN_URL=TODO - this needs to also work for app deeplinking
```

## Running Rust examples

The Rust examples live in `solari-rs/examples` and require feature flags.

Run from the workspace root:

```bash
# Stripe scenarios (requires Stripe env vars)
cargo run -p solari --example stripe_scenarios --features stripe

# Vipps scenarios (requires Vipps env vars)
cargo run -p solari --example vipps_scenarios --features vipps
```

Run from `solari-rs`:

```bash
cargo run --example stripe_scenarios --features stripe
cargo run --example vipps_scenarios --features vipps
```

Required env vars:

- Stripe: `STRIPE_SECRET_KEY`, `STRIPE_PUBLISHABLE_KEY`, `STRIPE_WEBHOOK_SECRET`
- Vipps: `VIPPS_BASE_URL`, `VIPPS_CLIENT_ID`, `VIPPS_CLIENT_SECRET`, `VIPPS_SUBSCRIPTION_KEY`, `VIPPS_MSN`

The examples load `.env` automatically via `dotenvy`. If required vars are missing, the scenario is skipped.
