## Apple Pay + Stripe setup

This project uses Stripe for both card and Apple Pay intents.

## 1) Get required Stripe keys

From Stripe Dashboard (Test mode):

- Developers -> API keys
- Copy `Publishable key` (`pk_test_...`)
- Copy `Secret key` (`sk_test_...`)

For local webhook testing:

- Run: `stripe listen --forward-to http://127.0.0.1:3001/solari/webhooks/stripe`
- Copy the printed signing secret (`whsec_...`) and set as `STRIPE_WEBHOOK_SECRET`

## 2) Configure env vars

In your workspace `.env`:

```bash
STRIPE_API_BASE_URL=https://api.stripe.com
STRIPE_SECRET_KEY=sk_test_...
STRIPE_PUBLISHABLE_KEY=pk_test_...
STRIPE_WEBHOOK_SECRET=whsec_...
# Optional for Connect:
# STRIPE_ACCOUNT_ID=acct_...
```

## 3) Apple Pay domain + merchant setup

Apple Pay on web requires domain verification in Stripe:

1. Stripe Dashboard -> Settings -> Payment methods -> Apple Pay.
2. Register your public HTTPS domain (for local dev, use ngrok domain).
3. Download `apple-developer-merchantid-domain-association` file.
4. Host the file at:
   `https://<your-domain>/.well-known/apple-developer-merchantid-domain-association`
5. Verify the domain in Stripe.

## 4) Apple Developer merchant ID (if needed for native)

If you also do native Apple Pay:

1. Apple Developer -> Certificates, IDs & Profiles -> Identifiers.
2. Create Merchant ID (`merchant.com.your-app`).
3. Create Apple Pay payment processing certificate and upload to Stripe if your flow requires it.

## 5) Test your backend routes

Card intent:

```bash
curl -X POST http://127.0.0.1:3001/solari/stripe/pay \
	-H 'content-type: application/json' \
	-d '{"amount":2500,"currency":"nok","description":"Card test"}'
```

Apple Pay intent:

```bash
curl -X POST http://127.0.0.1:3001/solari/apple-pay/pay \
	-H 'content-type: application/json' \
	-d '{"amount":2500,"currency":"nok","description":"Apple Pay test"}'
```

Both routes return `client_secret` + `publishable_key`, which frontend can use with Stripe.js or native SDKs.
