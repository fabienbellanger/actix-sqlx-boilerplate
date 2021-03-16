# actix-sqlx-boilerplate
A simple Actix-web boilerplate using SQLx


## Routes

### Web

- **[GET] `/health-check`**: Check server
    ```bash
    http GET localhost:8089/health-check
    ```
    Response code `200`

### API

- **[POST] `/v1/login`**: Authentication
    ```bash
    http POST localhost:8089/v1/login email=test@gmail.com password=0000
    ```
    Response code `200`:
    ```json
    {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "email": "test@gmail.com",
        "expires_at": "2021-03-08T21:23:21Z",
        "firstname": "Toto",
        "lastname": "Test",
        "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJpZCIsImV4cCI6MTYxNTIzODYwMSwiaWF0IjoxNjE1MjAyNjAxLCJuYmYiOjE2MTUyMDI2MDEsInVzZXJfaWQiOiJpZCIsInVzZXJfbGFzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2ZpcnN0bmFtZSI6IkZhYmllbiIsInVzZXJfZW1haWwiOiJ2YWxlbnRpbEBnbWFpbC5jb20ifQ.-rsxfNLJNIUwT1iZNy1X_9W6Ed0qAdMhTWmDujYaBNS-EOh5eCU-bXC98z7mXmfYxhTB7Vz7332geelrtbh98g"
    }
    ```

- **[POST] `/v1/register`**: User registration
    ```bash
    http POST localhost:8089/v1/register email=test@gmail.com password=0000 lastname=Test firstname=Toto
    ```
    Response code `200`:
    ```json
    {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "email": "test@gmail.com",
        "firstname": "Toto",
        "lastname": "Test",
        "created_at": "2021-03-05T11:29:55Z",
        "updated_at": "2021-03-05T12:30:02Z"
    }
    ```

- **[GET] `/v1/users`**: Users list
    ```bash
    http GET localhost:8089/v1/users "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
    ```
    Response code `200`:
    ```json
    [
        {
            "id": "123e4567-e89b-12d3-a456-426614174000",
            "lastname": "Test",
            "firstname": "Toto",
            "email": "test@gmail.com",
            "created_at": "2021-03-05T11:29:55Z",
            "updated_at": "2021-03-05T12:30:02Z"
        }
    ]
    ```

- **[GET] `/v1/users/{id}`**: Get user information
    ```bash
    http GET localhost:8089/v1/users/123e4567-e89b-12d3-a456-426614174000 "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
    ```
    Response code `200`:
    ```json
    {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "lastname": "Test",
        "firstname": "Toto",
        "email": "test@gmail.com",
        "created_at": "2021-03-05T11:29:55Z",
        "updated_at": "2021-03-05T12:30:02Z"
    }
    ```

- **[PUT] `/v1/users/{id}`**: Update user
    ```bash
    http PUT localhost:8089/v1/users/123e4567-e89b-12d3-a456-426614174000 "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU1NDk2OTksImlhdCI6MTYxNTQ2MzI5OSwibmJmIjoxNjE1NDYzMjk5LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.EajIFTzXLVjlaefDTeyoC5NKfz_MaPAhIIV4AG2cUwRE8tB35HecyJJukHk0kQXOJyMllgahttV2qpk6wGgD3g" lastname="Test" firstname="Tutu" email="test@gmail.com" password="1111"
    ```
    Response code `200`:
    ```json
    {
        "id": "123e4567-e89b-12d3-a456-426614174000",
        "lastname": "Test",
        "firstname": "Tutu",
        "email": "test@gmail.com",
        "created_at": "2021-03-05T11:29:55Z",
        "updated_at": "2021-03-06T12:30:02Z"
    }
    ```

- **[DELETE] `/v1/users/{id}`**: Delete user
    ```bash
    http DELETE localhost:8089/v1/users/123e4567-e89b-12d3-a456-426614174000 "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
    ```
  Response code `204`
  
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

**TODO:** Upgrade to sqlx 0.5 when actix-web upgraded in 4.0

### sqlx-cli
sqlx-cli repository: [Github](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

### Migrations
To create a migration:
```bash
sqlx migrate add -r <name>
```

## TODO list
-  [x] Implement JWT authentication
-  [x] Add `/register` route
-  [x] Mieux gérer la route /login quand le user est supprimé
-  [ ] Add SQLx auto-migrate. Done, but does not work with sqlx 0.4: 
    ```
    Error: 
    0: while executing migrations: error returned from database: 1062 (23000): Duplicate entry '20210316155055' for key 'PRIMARY'
    1: error returned from database: 1062 (23000): Duplicate entry '20210316155055' for key 'PRIMARY'
    ```
-  [ ] Watch [actix_sqlx_mysql_user_crud](https://github.com/jamesjmeyer210/actix_sqlx_mysql_user_crud)
-  [ ] Watch [actixweb-sqlx-jwt](https://github.com/biluohc/actixweb-sqlx-jwt/blob/master/src/middlewares/auth.rs)
