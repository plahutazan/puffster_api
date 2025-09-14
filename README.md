# To Do

- Wrap all API responses in a standard ApiResponse<T> for frontend consistency
- Input validation – check length, format, and required fields on signup/login.
- Timestamp logins/signups, everything.
- Consistent error codes – return HTTP status codes like 400, 401, 404, 500.
- Optional JWT claims – include roles or permissions in token payload.
- Structured error responses – include error_code, message, details.
- iIprove the /me with better sesion tokens

- Make a Flutter testing website. (branch)

- Add Game logic
- - growing logic
- - minigames logic

# Testing

### Signup
curl -X POST http://127.0.0.1:3000/signup -H "Content-Type: application/json" -d '{"display_name":"Zan","wallet":"0x123"}'

### Login
curl -X POST http://127.0.0.1:3000/login -H "Content-Type: application/json" -d '{"display_name":"Zan","wallet":"0x123"}'

### List Users
curl http://127.0.0.1:3000/users

### Me
curl -X POST http://127.0.0.1:3000/me -H "Content-Type: application/json" -d '"0x123"'
