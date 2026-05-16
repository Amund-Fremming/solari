# Apple Pay Setup: What You Need To Do

This is the minimum checklist to make Apple Pay work in Solari.

## 1. Create Apple Pay assets in Apple Developer

1. Sign in: https://developer.apple.com/account/
2. Create a Merchant ID.
3. Create a Payment Processing Certificate for that Merchant ID.
4. Create a Merchant Identity Certificate for merchant validation.
5. Verify your web domain for Apple Pay.

References:

1. https://developer.apple.com/documentation/apple_pay_on_the_web
2. https://developer.apple.com/help/account/capabilities/configure-apple-pay/

## 2. Add these values to .env

Use these keys in your environment file:

1. APPLE_PAY_MERCHANT_ID
2. APPLE_PAY_MERCHANT_DISPLAY_NAME
3. APPLE_PAY_INITIATIVE
4. APPLE_PAY_INITIATIVE_CONTEXT
5. APPLE_PAY_MERCHANT_VALIDATION_URL
6. APPLE_PAY_PAYMENT_PROCESSING_CERT_PEM
7. APPLE_PAY_PAYMENT_PROCESSING_KEY_PEM

Recommended defaults:

1. APPLE_PAY_INITIATIVE=web
2. APPLE_PAY_INITIATIVE_CONTEXT=your verified domain, for example checkout.example.com

## 3. Map env values into ApplePayConfig

Solari expects this mapping:

1. merchant_id <- APPLE_PAY_MERCHANT_ID
2. merchant_display_name <- APPLE_PAY_MERCHANT_DISPLAY_NAME
3. initiative <- APPLE_PAY_INITIATIVE
4. initiative_context <- APPLE_PAY_INITIATIVE_CONTEXT
5. merchant_validation_url <- APPLE_PAY_MERCHANT_VALIDATION_URL
6. payment_processing_cert_pem <- APPLE_PAY_PAYMENT_PROCESSING_CERT_PEM
7. payment_processing_key_pem <- APPLE_PAY_PAYMENT_PROCESSING_KEY_PEM

## 4. Wire backend setup

1. Build ApplePayConfig from env values.
2. Call payment_module.apple_pay(config) at startup.
3. Use PayRequest::ApplePay in your payment endpoint.

## 5. Important note for current Solari state

The Apple Pay provider in Solari is now structured like Vipps and validates required config, but it is still a backend scaffold. You still need to add your processor-specific token handling flow (Stripe/direct processor/etc.) and merchant session handling endpoint before production payments will capture funds.
