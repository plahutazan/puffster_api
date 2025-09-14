# To Do

- add signup token
- add uniqe name and wallet checking
- fix permissons with making the .db file

# Testing

## Signup
curl -X POST http://127.0.0.1:3000/signup \
  -H "Content-Type: application/json" \
  -d '{"display_name":"Zan","wallet":"jf4yhfuf3uy4hr4hf34t4"}'

## Login
curl -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{"display_name":"Alice","wallet":"0x123"}'

## List Users
curl http://127.0.0.1:3000/users