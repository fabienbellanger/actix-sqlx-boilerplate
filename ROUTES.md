# actix-sqlx-boilerplate
A simple Actix-web boilerplate using SQLx


## Routes

### Web

- `GET` `/health-check`: Check server
    ```bash
    http GET localhost:8089/health-check
    ```
    Response code `200`

- `GET` `/ws-client`: Web Sockets client
    ```bash
    http GET localhost:8089/ws-client
    ```
    Response code `200`

- `GET` `/ws-chat-client`: Chat Web Sockets client
    ```bash
    http GET localhost:8089/ws-chat-client
    ```
    Response code `200`

- `GET` `/actor-cache/{item}`: Chat Web Sockets client
    ```bash
    http GET localhost:8089/actor-cache/{item}
    ```
    Response code `200`:
    ```json
    [
        "toto",
        "titi"
    ]
    ```

- `GET` `/metrics`: Prometheus metrics
    ```bash
    http GET localhost:8089/metrics
    ```
    Response code `200`

### API

#### Authentication / Registration

- `POST` `/api/v1/login`: Authentication
    ```bash
    http POST localhost:8089/api/v1/login email=test@gmail.com password=00000000
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

- `POST` `/api/v1/register`: User registration
    ```bash
    http POST localhost:8089/api/v1/register email=test@gmail.com password=00000000 lastname=Test firstname=Toto
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

#### Users

- `GET` `/api/v1/users`: Users list
    ```bash
    http GET localhost:8089/api/v1/users "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
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

- `GET` `/api/v1/users/{id}`: Get user information
    ```bash
    http GET localhost:8089/api/v1/users/123e4567-e89b-12d3-a456-426614174000 "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
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

- `PUT` `/api/v1/users/{id}`: Update user
    ```bash
    http PUT localhost:8089/api/v1/users/123e4567-e89b-12d3-a456-426614174000 "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU1NDk2OTksImlhdCI6MTYxNTQ2MzI5OSwibmJmIjoxNjE1NDYzMjk5LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.EajIFTzXLVjlaefDTeyoC5NKfz_MaPAhIIV4AG2cUwRE8tB35HecyJJukHk0kQXOJyMllgahttV2qpk6wGgD3g" lastname="Test" firstname="Tutu" email="test@gmail.com" password="1111"
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

- `DELETE` `/api/v1/users/{id}`: Delete user
    ```bash
    http DELETE localhost:8089/api/v1/users/123e4567-e89b-12d3-a456-426614174000 "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjNlNDU2Ny1lODliLTEyZDMtYTQ1Ni00MjY2MTQxNzQwMDAiLCJleHAiOjE2MTU0NDg2ODQsImlhdCI6MTYxNTM2MjI4NCwibmJmIjoxNjE1MzYyMjg0LCJ1c2VyX2lkIjoiMTIzZTQ1NjctZTg5Yi0xMmQzLWE0NTYtNDI2NjE0MTc0MDAwIiwidXNlcl9sYXN0bmFtZSI6ImM2MDAxZDViMmFjM2RmMzE0MjA0YThmOWQ3YTAwZTE1MDNjOWFiYTBmZDQ1Mzg2NDVkZTRiZjRjYzdlMjU1NWNmZTlmZjlkMDIzNmJmMzI3ZWQzZTkwNzg0OWE5OGRmNGQzMzBjNGJlYTU1MTAxN2Q0NjViNGMxZDliODBiY2IwIiwidXNlcl9maXJzdG5hbWUiOiJCZWxsYW5nZXIiLCJ1c2VyX2VtYWlsIjoiRmFiaWVuIn0.UkJ_5KEIhs--Hv8cfggEpb8xxv2UhiwjRQTIlNmudZ8h-XHlikev4fwXU7N9wbP1esIHlo2_tafPyjnGxCBscQ"
    ```
  Response code `204`

#### Tasks

- `GET` `/api/v1/tasks`: Tasks list
    ```bash
    http GET localhost:8089/api/v1/tasks "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJhMjRmZmMxZi1mYWE3LTRkYTktOTNhOC0xNDc0M2Y4YmJmM2YiLCJleHAiOjE2MjM3OTU4MzgsImlhdCI6MTYyMzc4ODYzOCwibmJmIjoxNjIzNzg4NjM4LCJ1c2VyX2lkIjoiYTI0ZmZjMWYtZmFhNy00ZGE5LTkzYTgtMTQ3NDNmOGJiZjNmIiwidXNlcl9sYXN0bmFtZSI6ImNlMmE0MjlhMWM3OWQ0MDY4YzBjN2U1NGY1NTAwY2UxNjI4NWQ4NTczMGNiOWVjMGI2MTI0MGY4OGVmOWM4NzAyOTIyMDBhMWMwNjliZDU3ZDVlMDkyODc0NTY3MDU4YzkxNzgyNTEzNzYzYmMzMGQ4NmZlZGNhNjM4MjBjNDgyIiwidXNlcl9maXJzdG5hbWUiOiJUZXN0IiwidXNlcl9lbWFpbCI6IlRvdG8ifQ.6QDio25aB3jO__VbS0S-0LlIsVgNyDYsSn7-xWLwIwm5v0pubHS4JT-ToicWuDmaV1lG39DF2V_OLCbmKKQjyQ"
    ```
    Response code `200`:
    ```json
    [
        {
            "id": "d4ad12fe-dcac-443f-8048-f05aa88d6e25",
            "name": "Task name",
            "description": "A long task description...",
            "created_at": "2021-06-15T20:24:18Z",
            "updated_at": "2021-06-15T20:24:18Z"
        },
        {
            "id": "fa29c383-3db7-4253-bd6c-b51045fe0f12",
            "name": "Task name without description",
            "description": null,
            "created_at": "2021-06-15T20:24:42Z",
            "updated_at": "2021-06-15T20:24:42Z"
        }
    ]
    ```

- `POST` `/api/v1/tasks`: Task creation
    ```bash
    http POST localhost:8089/api/v1/tasks name="My Task" "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJhMjRmZmMxZi1mYWE3LTRkYTktOTNhOC0xNDc0M2Y4YmJmM2YiLCJleHAiOjE2MjM3OTU4MzgsImlhdCI6MTYyMzc4ODYzOCwibmJmIjoxNjIzNzg4NjM4LCJ1c2VyX2lkIjoiYTI0ZmZjMWYtZmFhNy00ZGE5LTkzYTgtMTQ3NDNmOGJiZjNmIiwidXNlcl9sYXN0bmFtZSI6ImNlMmE0MjlhMWM3OWQ0MDY4YzBjN2U1NGY1NTAwY2UxNjI4NWQ4NTczMGNiOWVjMGI2MTI0MGY4OGVmOWM4NzAyOTIyMDBhMWMwNjliZDU3ZDVlMDkyODc0NTY3MDU4YzkxNzgyNTEzNzYzYmMzMGQ4NmZlZGNhNjM4MjBjNDgyIiwidXNlcl9maXJzdG5hbWUiOiJUZXN0IiwidXNlcl9lbWFpbCI6IlRvdG8ifQ.6QDio25aB3jO__VbS0S-0LlIsVgNyDYsSn7-xWLwIwm5v0pubHS4JT-ToicWuDmaV1lG39DF2V_OLCbmKKQjyQ"
    ```
    Response code `200`:
    ```json
    {
        "id": "fa29c383-3db7-4253-bd6c-b51045fe0f12",
        "name": "My Task",
        "description": null,
        "created_at": "2021-06-15T20:24:42.468853Z",
        "updated_at": "2021-06-15T20:24:42.468858Z"
    }
    ```