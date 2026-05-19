# next-test

Next.js sandbox for Solari web Vipps integration.

## Stripe SDK flow

This example now includes Stripe.js via `@stripe/react-stripe-js`.

- Use the "Card Intent" or "Apple Pay Intent" button to create a payment intent from Solari backend.
- Confirm the payment inside the embedded Stripe Payment Element.
- For Apple Pay to appear on web, Stripe domain verification and Apple Pay browser/device support are required.

The Next app reads API base URL from `NEXT_PUBLIC_AXUM_BASE_URL`.

## Local run

1. Start the Axum backend from workspace root:

```bash
cargo run -p axum-test
```

2. Configure the backend URL for this app:

```bash
NEXT_PUBLIC_AXUM_BASE_URL=http://127.0.0.1:3001
```

3. Start the Next.js app:

```bash
npm run dev
```

## When approving payment on phone

If you open the Vipps flow in browser and confirm payment from the Vipps app on a phone, the callback URL must be publicly reachable over HTTPS.

Use an ngrok domain for the backend and set:

```bash
NEXT_PUBLIC_AXUM_BASE_URL=https://your-ngrok-domain.ngrok-free.app
```

Optional: if you host the Next.js app on a public HTTPS URL and want Vipps to redirect there after payment, set:

```bash
NEXT_PUBLIC_VIPPS_WEB_RETURN_URL=https://your-web-host/vipps-return
```

Without a public URL, phone-confirmed callbacks to `localhost` or `127.0.0.1` will fail.
