### Login
```
curl --insecure -X POST -i 'https://127.0.0.1:9000/api/auth/login'   -H "Content-Type: application/json"   --data '{
    "email": "a1",
    "password": "12345"
  }'
```

### Place bet / cancel bet
```
curl --insecure -X POST  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTk5MDk0MTMsImV4cCI6MTYyMDUxNDIxMywidXNlciI6ImExIiwibG9naW5fc2Vzc2lvbiI6IjE0MGNlZWM5Mjg4ZjQ0Mzk5MDlmZTFmZmFkYWYyOThmIn0.-G6U-GEu6TQ7ah9darFcQRifdSPLAEU8DbvS1zUjvhw'  -H "Content-Type: application/json" -i 'https://127.0.0.1:9000/api/bet/cancel' --data '{
    "monster": 2,
    "gold": 10000
  }'

 curl --insecure -X POST  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTk5MDk0MTMsImV4cCI6MTYyMDUxNDIxMywidXNlciI6ImExIiwibG9naW5fc2Vzc2lvbiI6IjE0MGNlZWM5Mjg4ZjQ0Mzk5MDlmZTFmZmFkYWYyOThmIn0.-G6U-GEu6TQ7ah9darFcQRifdSPLAEU8DbvS1zUjvhw'  -H "Content-Type: application/json" -i 'https://127.0.0.1:9000/api/bet/place' --data '{
    "monster": 2,
    "gold": 10000
  }'

```

### Deposit
```
curl --insecure -X POST  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE2MTkxODE5NjQsImV4cCI6MTYxOTc4Njc2NCwidXNlciI6ImExIiwibG9naW5fc2Vzc2lvbiI6ImU2YWE4MjI3MWM3ZjQxZGVhYzUyYzk0ZTI4MzI4MmRmIn0.RcaKVaoF-5XorR6N2Bo3ZTjhnNkN1Jz6UAuTLCPzO-c'  -H "Content-Type: application/json" -i 'https://127.0.0.1:9000/api/gold/deposit' --data '{
    "email": "a2",
    "gold": 1000
  }'
```
### Signup
```
curl --insecure -X POST -i 'https://127.0.0.1:9000/api/auth/signup'   -H "Content-Type: application/json"   --data '{
    "invite_code": "712534",
    "email": "a1b",
    "password": "12345"
  }'
```

