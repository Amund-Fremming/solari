# expo-test

Placeholder app for Expo/React Native integration tests with Solari.

## Vipps flow

Set the backend URL before starting Expo when testing on a real device:

```bash
EXPO_PUBLIC_AXUM_BASE_URL=https://your-ngrok-domain.ngrok-free.app npm start
```

If you run the iOS simulator locally, the app falls back to `http://127.0.0.1:3001`.
If you run the Android emulator locally, the app falls back to `http://10.0.2.2:3001`.

## Suggested bootstrap

```bash
npx create-expo-app@latest .
```
