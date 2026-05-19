# expo-test

Placeholder app for Expo/React Native integration tests with Solari.

## Vipps flow

Set the backend URL before starting Expo when testing on a real device:

```bash
EXPO_PUBLIC_AXUM_BASE_URL=https://your-ngrok-domain.ngrok-free.app npm start
```

If you run the iOS simulator locally, the app falls back to `http://127.0.0.1:3001`.
If you run the Android emulator locally, the app falls back to `http://10.0.2.2:3001`.

For real-device Vipps tests, keep the backend on an HTTPS public URL (for example ngrok).
The mobile flow now sends Vipps back to `${EXPO_PUBLIC_AXUM_BASE_URL}/vipps-return`,
and that endpoint deep-links into `solari-expo-test://vipps-return` so Expo can resume the auth session.

## Suggested bootstrap

```bash
npx create-expo-app@latest .
```

## Stripe SDK flow

This example now uses `@stripe/stripe-react-native` and PaymentSheet.

Set these env vars before starting Expo:

```bash
EXPO_PUBLIC_AXUM_BASE_URL=https://your-ngrok-domain.ngrok-free.app
EXPO_PUBLIC_STRIPE_PUBLISHABLE_KEY=pk_test_...
npm start
```

Notes:

- The app config includes Stripe plugin with merchant identifier `merchant.com.solari.test`.
- For production Apple Pay, use your own merchant identifier and Apple Pay-capable iOS setup.
