# actix-sqlx-boilerplate
A simple Actix-web boilerplate using SQLx


## Routes

### Web

- **[GET] `/health-check`**: Check server
    ```bash
    http GET localhost:8089/health-check
    ```

### API

- **[POST] `/v1/login`**: Authentication
    ```bash
    http POST localhost:8089/v1/login email=valentil@gmail.com password=0000
    ```
    Response:
    ```json
    {
        "email": "valentil@gmail.com",
        "expires_at": "2021-03-08T21:23:21Z",
        "firstname": "Fabien",
        "lastname": "Bellanger",
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJpZCIsImV4cCI6MTYxNTIzODYwMSwiaWF0IjoxNjE1MjAyNjAxLCJuYmYiOjE2MTUyMDI2MDEsInVzZXJfaWQiOiJpZCIsInVzZXJfbGFzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2ZpcnN0bmFtZSI6IkZhYmllbiIsInVzZXJfZW1haWwiOiJ2YWxlbnRpbEBnbWFpbC5jb20ifQ.-rsxfNLJNIUwT1iZNy1X_9W6Ed0qAdMhTWmDujYaBNS-EOh5eCU-bXC98z7mXmfYxhTB7Vz7332geelrtbh98g"
    }
    ```

- **[GET] `/v1/users`**: Users list
    ```bash
    http GET localhost:8089/v1/users "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
    ```
    Response:
    ```json
    [
        {
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "lastname": "Bellanger",
            "firstname": "Fabien",
            "email": "valentil@gmail.com",
            "created_at": "2021-03-05T11:29:55Z",
            "updated_at": "2021-03-05T12:30:02Z"
        }
    ]
    ```

- **[GET] `/v1/users/{id}`**: Get user information
    ```bash
    http GET localhost:8089/v1/users/123e4567-e89b-12d3-a456-426614174000 "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
    ```
    Response:
    ```json
    {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "lastname": "Bellanger",
        "firstname": "Fabien",
        "email": "valentil@gmail.com",
        "created_at": "2021-03-05T11:29:55Z",
        "updated_at": "2021-03-05T12:30:02Z"
    }
    ```
  
## Logs
Use of [bunyan](https://crates.io/crates/bunyan)
```bash
cargo run | bunyan
```

## Cargo watch
Usage:
```bash
cargo watch -x 'run --bin actix-web'
```

## Benchmark
Use [Drill](https://github.com/fcsonline/drill)
```bash
$ drill --benchmark drill.yml --stats --quiet
```

## SQLx
sqlx repository: [Github](https://github.com/launchbadge/sqlx)

**TODO:** Passer à sqlx 0.5 quand actix-web passera en 4.0

### sqlx-cli
sqlx-cli repository: [Github](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

### Migrations
To create a migration:
```bash
sqlx migrate add -r <name>
```

## TODO list
-  [x] Implement JWT auth
-  [ ] Faire la route /register
-  [ ] Mieux gérer la route /login quand le user est supprimé
-  [ ] Add custom Tracing formatter
-  [ ] Add SQLx auto-migrate
-  [ ] Regarder [actix_sqlx_mysql_user_crud](https://github.com/jamesjmeyer210/actix_sqlx_mysql_user_crud)
-  [ ] Regarder [actixweb-sqlx-jwt](https://github.com/biluohc/actixweb-sqlx-jwt/blob/master/src/middlewares/auth.rs)
