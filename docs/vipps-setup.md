# Vipps Setup

## Test Environment

1. Gå til bedriftsportalen.
2. Velg bedriften du har opprettet.
3. Velg For utviklere nederst til venstre i menyen.
4. Generer testbruker.
5. Last ned Vipps test-app og fyll inn data for testbrukeren.
6. Kode på mobil er `1236`.

## Payment Flow

1. User taps the pay button.
2. Frontend initiates API call to backend.
3. Backend uses payment provider with Vipps, receives a URL, and passes it to Vipps flow in client.

## Testing Notes

- For deeplinking tests, use ngrok on backend.
- Use the ngrok domain in the npm client.
- Test on a physical phone with Vipps test-app installed.
