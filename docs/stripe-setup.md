# Stripe Setup

## Overview

This guide covers Stripe sandbox setup and Expo environment configuration.

## Sandbox Setup

1. Log in to Stripe sandbox.
2. Copy the publishable key and secret key from the right side of the dashboard.
3. Use the Stripe base URL:

```txt
https://api.stripe.com
```

## Expo Configuration

Set the following environment variables:

```env
EXPO_PUBLIC_STRIPE_PUBLISHABLE_KEY=pk_test_your_publishable_key
EXPO_PUBLIC_STRIPE_MERCHANT_IDENTIFIER=merchant.com.apple-pay-test-amund
```

## Important Note

When creating the merchant identifier, make sure you accept the required terms.
