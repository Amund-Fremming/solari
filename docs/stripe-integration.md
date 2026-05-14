# Stripe Setup: What To Do

## 1. Where to go

Use these official pages:

1. Stripe docs home: https://docs.stripe.com/
2. API keys: https://docs.stripe.com/keys
3. Payment Intents API: https://docs.stripe.com/payments/payment-intents
4. Apple Pay with Stripe: https://docs.stripe.com/apple-pay
5. Webhooks: https://docs.stripe.com/webhooks
6. Testing: https://docs.stripe.com/testing

## 2. What to create in Stripe

Start in test mode:

1. Create a Stripe account and activate dashboard access.
2. Get test API keys.
3. Create a webhook endpoint for your backend.
4. Enable any needed payment methods in dashboard.

## 3. Values you must store in StripeConfig

Store these values:

1. api_base_url
2. secret_key
3. publishable_key
4. webhook_secret
5. account_id (optional, only for Connect/platform flows)

## 4. Suggested env vars

1. STRIPE_API_BASE_URL
2. STRIPE_SECRET_KEY
3. STRIPE_PUBLISHABLE_KEY
4. STRIPE_WEBHOOK_SECRET
5. STRIPE_ACCOUNT_ID

## 5. Payment flow

1. Create PaymentIntent on backend.
2. Confirm on client or server depending on payment method.
3. Listen for webhook events for final state.
4. Reconcile order state from webhook, not only client callback.
