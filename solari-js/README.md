# @solari/solari-js

Minimal Solari JavaScript client.

- One main class: `SolariPaymentService`
- Uses `fetch`
- Typed request/response packets in `types.ts`
- Includes Vipps button support for web and native

## Install

```bash
npm install @solari/solari-js
```

## Quick start

```ts
import { SolariPaymentService, type VippsPayPacket } from "@solari/solari-js";

const solari = new SolariPaymentService({
  baseUrl: "http://127.0.0.1:3001",
});

const packet: VippsPayPacket = {
  amount: 67,
  return_url: "https://your-app.example/vipps-return",
};

const response = await solari.vippsPay(packet);
console.log(response.redirect_url);
```

Stripe card payment intent:

```ts
const stripeIntent = await solari.stripePay({
  amount: 2500,
  currency: "nok",
  description: "Card payment",
});

console.log(stripeIntent.payment_intent_id, stripeIntent.client_secret);
```

Apple Pay payment intent:

```ts
const applePayIntent = await solari.applePayPay({
  amount: 2500,
  currency: "nok",
  description: "Apple Pay payment",
});

console.log(applePayIntent.flow, applePayIntent.client_secret);
```

## Main class

`SolariPaymentService` has grouped endpoint methods:

- Axum test endpoints: `pay`, `getPaymentStatus`, `resetPayment`
- Vipps endpoints: `vippsPay`, `vippsGetToken`, `vippsFetchToken`, `vippsCreatePayment`, `startVippsPayment`
- Apple Pay endpoint: `applePayPay`
- Stripe endpoint: `stripePay`

## Types

All API packets and response types are exported from `@solari/solari-js` and `@solari/solari-js/native`.

Examples:

- `VippsPayPacket`
- `VippsCreatePaymentPacket`
- `StripePayPacket`
- `StripePaymentIntentResponse`
- `PaymentSnapshot`
- `PaymentApiResponse`

## Vipps button assets

Native:

- Use `VippsButtonNative` from `@solari/solari-js/native`
- Renders the packaged SVG button via `react-native-svg`

Web:

- Use `VippsButtonWeb` from `@solari/solari-js`
- Or get direct SVG URL with `getVippsButtonSvgUrl()`
