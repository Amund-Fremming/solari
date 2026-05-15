# Vipps Production and Go-Live Setup

Use this guide when you are ready for real payments.

## 1. Prerequisites

1. Registered business with organization number.
2. Approved Vipps merchant agreement.
3. Sales unit ready for production use.
4. ePayment enabled for that sales unit.

## 2. Production setup in Vipps

1. Select the production sales unit.
2. Generate production API keys.
3. Confirm production subscription key is available.
4. Verify callback URLs point to your production backend.

Reference pages:

1. Portal usage: https://developer.vippsmobilepay.com/docs/knowledge-base/portal/
2. API keys: https://developer.vippsmobilepay.com/docs/knowledge-base/api-keys/
3. Authentication: https://developer.vippsmobilepay.com/docs/knowledge-base/authentication/
4. Servers: https://developer.vippsmobilepay.com/docs/knowledge-base/servers/

## 3. Config values to store

Store these values in your production config:

1. base_url (production server URL from servers doc)
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

## 5. Go-live checklist

1. Ensure test environment scenarios are completed.
2. Enable strict logging and monitoring around payment endpoints.
3. Confirm idempotency handling on create/capture/refund.
4. Verify callback signature/validation flow.
5. Roll out to production with feature flags if possible.
6. Monitor first real transactions closely.