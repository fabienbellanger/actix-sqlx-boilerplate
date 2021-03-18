# actix-sqlx-boilerplate
A simple Actix-web boilerplate using SQLx


## Routes
[Documentation](ROUTES.md)
  
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
cargo run --open --no-deps
```

Run with private items:
```bash
cargo run --open --no-deps --document-private-items
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
