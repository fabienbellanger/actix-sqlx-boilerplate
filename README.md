# actix-sqlx-boilerplate
A simple Actix-web boilerplate using SQLx


## Routes
[Documentation](ROUTES.md)
  
## Run
```bash
cargo run --bin api
```

## Logs
The project uses tracing for logs.  
To format them, the project [actix-web-logs-reader](https://github.com/fabienbellanger/actix-web-logs-reader) which is inspired by Luca Palmieri's [Bunyan project](https://github.com/LukeMathWalker/bunyan).  
To install it, just run:
```bash
cargo install --git https://github.com/fabienbellanger/actix-web-logs-reader --branch main
```
Actix-web logs can be formatted like this:
```bash
cargo run | actix-web-logs-reader
```

## Cargo watch
cargo-watch repository: [Github](https://github.com/passcod/cargo-watch)
Usage:
```bash
cargo watch -x 'run --bin api'
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
Run migrations:
```bash
sqlx migrate run
```
Revet migrations:
```bash
sqlx migrate revert
```

## Documentation
Run:
```bash
cargo doc --open --no-deps
```

Run with private items:
```bash
cargo doc --open --no-deps --document-private-items
```

## TODO list
-  [x] Implement JWT authentication
-  [x] Add `/register` route
-  [x] Mieux gérer la route /login quand le user est supprimé
-  [x] Implement FromRequest trait to retrieve the request ID in an handler
-  [ ] Add SQLx auto-migrate. Done, but does not work with sqlx 0.4: 
    ```
    Error: 
    0: while executing migrations: error returned from database: 1062 (23000): Duplicate entry '20210316155055' for key 'PRIMARY'
    1: error returned from database: 1062 (23000): Duplicate entry '20210316155055' for key 'PRIMARY'
    ```
-  [ ] Watch [actix_sqlx_mysql_user_crud](https://github.com/jamesjmeyer210/actix_sqlx_mysql_user_crud)
-  [ ] Watch [actixweb-sqlx-jwt](https://github.com/biluohc/actixweb-sqlx-jwt/blob/master/src/middlewares/auth.rs)
-  [ ] Improve validator response error
