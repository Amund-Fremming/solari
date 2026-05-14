# Apple Pay Setup: What To Do

## 1. Where to go

Use these official pages:

1. Apple Developer Account: https://developer.apple.com/account/
2. Apple Pay on the web overview: https://developer.apple.com/documentation/apple_pay_on_the_web
3. Merchant IDs and certificates: https://developer.apple.com/help/account/capabilities/configure-apple-pay/
4. Stripe Apple Pay guide (if using Stripe as processor): https://docs.stripe.com/apple-pay

## 2. What to create in Apple

For Apple Pay on the web, create and configure:

1. Merchant ID
2. Payment Processing Certificate
3. Merchant Identity Certificate (for merchant validation)
4. Verified merchant domain

If using Stripe, complete Stripe Apple Pay setup too.

## 3. Values you must store in ApplePayConfig

Store these values:

1. merchant_id
2. merchant_display_name
3. initiative
4. initiative_context
5. merchant_validation_url
6. payment_processing_cert_pem
7. payment_processing_key_pem

Notes:

1. initiative is usually web.
2. initiative_context is usually your verified domain.

## 4. Suggested env vars

1. APPLE_PAY_MERCHANT_ID
2. APPLE_PAY_MERCHANT_DISPLAY_NAME
3. APPLE_PAY_INITIATIVE
4. APPLE_PAY_INITIATIVE_CONTEXT
5. APPLE_PAY_MERCHANT_VALIDATION_URL
6. APPLE_PAY_PAYMENT_PROCESSING_CERT_PEM
7. APPLE_PAY_PAYMENT_PROCESSING_KEY_PEM

## 5. Payment flow

1. Frontend checks Apple Pay availability.
2. Backend performs merchant validation.
3. Frontend returns payment token.
4. Backend decrypts/processes token directly or via provider (for example Stripe).
5. Backend finalizes payment and updates order state.
