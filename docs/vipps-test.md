# Vipps Test and Sandbox Setup

Use this guide when developing and testing before go-live.

## 1. Purpose of this environment

1. Validate your token flow.
2. Validate create payment and capture/refund flow.
3. Test callback handling and idempotency behavior.

## 2. Required setup in Vipps

1. Sign in to the Vipps/MobilePay business portal.
2. Create or select a sales unit in test context.
3. Enable ePayment for that sales unit.
4. Generate test API keys.
5. Confirm you have the correct test subscription key.

Reference pages:

1. Test environment: https://developer.vippsmobilepay.com/docs/knowledge-base/test-environment/
2. Portal usage: https://developer.vippsmobilepay.com/docs/knowledge-base/portal/
3. API keys: https://developer.vippsmobilepay.com/docs/knowledge-base/api-keys/
4. Servers: https://developer.vippsmobilepay.com/docs/knowledge-base/servers/

## 3. Config values to store

Store these values in your test config:

1. base_url (test server URL from servers doc)
2. client_id
3. client_secret
4. subscription_key
5. merchant_serial_number

Optional but useful:

1. fallback_system_name
2. fallback_system_version
3. plugin_name
4. plugin_version

## 4. Suggested env vars

1. VIPPS_BASE_URL
2. VIPPS_CLIENT_ID
3. VIPPS_CLIENT_SECRET
4. VIPPS_SUBSCRIPTION_KEY
5. VIPPS_MERCHANT_SERIAL_NUMBER

## 5. Test flow checklist

1. Exchange keys for access token.
2. Cache token until near expiry.
3. Create payment with idempotency key.
4. Retry same operation with same idempotency key if needed.
5. Verify test response and status transitions.
6. Test callback endpoint handling.

## 6. If you do not have business access yet

1. Continue app development with mocked Vipps responses.
2. Keep the same request/response shapes as the real API.
3. Switch env vars to real test credentials later.