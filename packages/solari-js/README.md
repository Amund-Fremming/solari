# @solari/solari-js

JavaScript helpers for Solari integrations.

## Install

```bash
npm install @solari/solari-js
```

## Vipps client setup

Create the client once at startup and pass callback/base URL there.

### Web

```ts
import { createWebClient } from "@solari/solari-js";

const vippsClient = createWebClient({
  apiBaseUrl: process.env.NEXT_PUBLIC_AXUM_BASE_URL,
  callbackUrl: process.env.NEXT_PUBLIC_VIPPS_WEB_RETURN_URL,
});

await vippsClient.startVippsPayment();
```

### Native (Expo)

```ts
import { Linking } from "react-native";
import { createNativeClient } from "@solari/solari-js";

const vippsClient = createNativeClient({
  apiBaseUrl: process.env.EXPO_PUBLIC_AXUM_BASE_URL,
  callbackUrl: "solari-expo-test://vipps-return",
  openUrl: (url) => Linking.openURL(url),
});

await vippsClient.startVippsPayment();
```

Behavior:

- Both web and native use one callback flow through backend: `<AXUM_BASE_URL>/vipps-return?app_return_url=<callbackUrl>`.
- If `apiBaseUrl` is omitted, SDK falls back to `NEXT_PUBLIC_AXUM_BASE_URL`/`EXPO_PUBLIC_AXUM_BASE_URL` and then local defaults.
- If web `callbackUrl` is omitted, SDK uses `NEXT_PUBLIC_VIPPS_WEB_RETURN_URL` when available, otherwise `<window.origin>/vipps-return`.

For phone-confirmed web payments, use public HTTPS URLs (for example ngrok) for both backend and callback URLs.
