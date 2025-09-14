# To Do

# Testing

### Signup
curl -X POST http://127.0.0.1:3000/signup -H "Content-Type: application/json" -d '{"display_name":"Zan","wallet":"0x123"}'

### Login
curl -X POST http://127.0.0.1:3000/login -H "Content-Type: application/json" -d '{"display_name":"Zan","wallet":"0x123"}'

### List Users
curl http://127.0.0.1:3000/users

### Me
curl -X POST http://127.0.0.1:3000/me -H "Content-Type: application/json" -d '"0x123"'
