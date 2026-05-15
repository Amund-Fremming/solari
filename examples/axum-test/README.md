# axum-test

Axum test backend for Solari.

## Endpoints

- `POST /pay`: attempts a fixed `67 NOK` Vipps payment and stores the latest state in memory.
- `GET /vipps-return`: browser return bridge that deep-links back to the Expo app.
- `GET /status`: returns the latest in-memory payment state.
- `POST /wipe`: resets the in-memory payment state.
- `POST /webhook/vipps`: accepts Vipps webhook JSON and updates the in-memory payment state.

## Example flow

Start the server:

```bash
cargo run -p axum-test
```

Trigger the fixed 67 NOK Vipps payment:

```bash
curl -X POST http://127.0.0.1:3001/pay
```

Check the in-memory payment state:

```bash
curl http://127.0.0.1:3001/status
```

Simulate a Vipps webhook callback locally:

```bash
curl -X POST http://127.0.0.1:3001/webhook/vipps \
	-H 'content-type: application/json' \
	-d '{"eventType":"PAYMENT.AUTHORIZED"}'
```

Reset the in-memory payment state:

```bash
curl -X POST http://127.0.0.1:3001/wipe
```

## Run

1. Install ngrok and authenticate once (`ngrok config add-authtoken <token>`) if needed.
2. Put your Vipps and ngrok values in the workspace root `.env`.
3. Start the server from workspace root:

```bash
cargo run -p axum-test
```

If `NGROK_ENABLED=true`, the app will spawn `ngrok http <AXUM_PORT>` automatically on startup.
If `NGROK_DOMAIN` is set, startup logs also print the exact public URLs for `/pay`, `/status`, `/wipe`, and `/webhook/vipps`.

## Environment variables

These are read from the workspace root `.env`.

- `AXUM_PORT`: local server port. Default is `3001`.
- `VIPPS_BASE_URL`: Vipps API base URL, usually `https://apitest.vipps.no`.
- `VIPPS_CLIENT_ID`: Vipps client id.
- `VIPPS_CLIENT_SECRET`: Vipps client secret.
- `VIPPS_SUBSCRIPTION_KEY`: Vipps subscription key.
- `VIPPS_MSN`: Vipps merchant serial number.
- `NGROK_ENABLED`: `true`/`false`. Default is `false`.
- `NGROK_DOMAIN`: optional reserved public URL for stable public URL.
- `NGROK_AUTHTOKEN`: optional token passed to ngrok process.

## Ngrok webhook setup

If you use a reserved ngrok URL such as `https://my-solari-dev.ngrok-free.app`, set it in `.env`:

```bash
NGROK_ENABLED=true
NGROK_DOMAIN=https://my-solari-dev.ngrok-free.app
NGROK_AUTHTOKEN=...
```

Then configure Vipps to call:

```text
https://my-solari-dev.ngrok-free.app/webhook/vipps
```

## Mobile testing notes

For real-device Expo testing, use your ngrok HTTPS domain as your backend base URL in the mobile app.
