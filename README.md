# actix-sqlx-boilerplate
A simple Actix-web boilerplate using SQLx


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
-  [ ] Implement JWT auth
-  [ ] Add custom Tracing formatter
-  [ ] Add SQLx auto-migrate