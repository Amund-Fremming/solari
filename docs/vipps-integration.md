# Vipps Setup: What To Do

## 1. Where to go

Use these official pages:

1. Developer home: https://developer.vippsmobilepay.com/
2. Business portal: https://developer.vippsmobilepay.com/docs/knowledge-base/portal/
3. API keys: https://developer.vippsmobilepay.com/docs/knowledge-base/api-keys/
4. Authentication: https://developer.vippsmobilepay.com/docs/knowledge-base/authentication/
5. Access token API: https://developer.vippsmobilepay.com/docs/APIs/access-token-api/
6. ePayment quick start: https://developer.vippsmobilepay.com/docs/APIs/epayment-api/quick-start/
7. API servers (test/prod URLs): https://developer.vippsmobilepay.com/docs/knowledge-base/servers/
8. Test environment: https://developer.vippsmobilepay.com/docs/knowledge-base/test-environment/

## 2. What to create in Vipps

In test first, then production:

1. Create or select your merchant sales unit.
2. Enable ePayment for the sales unit.
3. Generate merchant API keys.
4. Confirm the correct API product subscription key is available.

## 3. Values you must store in VippsConfig

Store these values:

1. base_url
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

## 5. Token and payment flow

1. Exchange keys for access token via Access Token API.
2. Cache token until near expiry.
3. Call ePayment endpoints using Authorization: Bearer token.
4. Reuse idempotency key on retries of the same payment operation.
