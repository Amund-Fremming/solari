# test env

- bedriftsportalen
- velg bedriften du har opprettet
- nederst venstre i menyen velg for utviklere
- generer test bruker
- last ned vipps test app og fyll inn data for en test bruker
- kode på mobil er 1236

flow

1. user hits pay on button
2. initiate api call to backend
3. use the payment provider to use vipps as payment, you get a url back, use this to feed the vipps stuff ...

## notes

- testing deeplinking requires you to use ngrok on backend, and use the ngrok domain in the npm client and test the vipps payment on a actual phone that has the vipps test app installed.
