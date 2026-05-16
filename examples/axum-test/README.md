# axum-test

Super-slim Axum test backend for Solari.

## Endpoints

- `POST /pay`: creates a payment through `SolariPaymentService::pay` (generic pay API).
- `GET /status`: fetches live status from Vipps using the last payment reference.
- `POST /wipe`: clears local test state (last payment reference + cached snapshot).
- `POST /solari/vipps/pay`: your Solari API route (mounted from `solari-client`).
- `GET /solari/vipps/token`: your Solari API route.
- `POST /solari/vipps/token/fetch`: your Solari API route.
- `POST /solari/vipps/payments`: your Solari API route.

## Example flow

Start the server:

```bash
cargo run -p axum-test
```

Trigger payment (defaults to `67 NOK` if amount is omitted):

```bash
curl -X POST http://127.0.0.1:3001/pay \
	-H 'content-type: application/json' \
	-d '{"amount":67}'
```

Fetch latest status using your API-backed status lookup:

```bash
curl http://127.0.0.1:3001/status
```

Reset all local test state:

```bash
curl -X POST http://127.0.0.1:3001/wipe
```

Call your native Solari API directly:

```bash
curl -X POST http://127.0.0.1:3001/solari/vipps/pay \
	-H 'content-type: application/json' \
	-d '{"amount":67}'
```

## Run

1. Install ngrok and authenticate once (`ngrok config add-authtoken <token>`) if needed.
2. Put your Vipps and ngrok values in the workspace root `.env`.
3. Start the server from workspace root:

```bash
cargo run -p axum-test
```

If `NGROK_ENABLED=true`, the app will spawn `ngrok http <AXUM_PORT>` automatically on startup.

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
https://my-solari-dev.ngrok-free.app/solari/vipps/payments
```

## Mobile testing notes

For real-device Expo testing, use your ngrok HTTPS domain as your backend base URL in the mobile app.

For web testing where payment is approved on a phone, `return_url` must also be publicly reachable over HTTPS (for example the same ngrok domain). A localhost callback (`127.0.0.1` / `localhost`) cannot be opened from the phone.
