# northwind-rs

The Northwind application powered by Rust (actix-web, sqlx, jwt...)

Generated from [`actix-sqlx-boilerplate`](https://github.com/fabienbellanger/actix-sqlx-boilerplate)

## Give a star ⭐

If you're using this repository for your samples, workshop, your project or whatever, please give a star ⭐. Thank you very much :+1:

# Up and running

## Run server

```bash
$ docker-compose up
$ cargo run --bin api
```

Then play around with `restclient.http`

# SQLx

sqlx repository: [Github](https://github.com/launchbadge/sqlx)
## sqlx-cli

sqlx-cli repository: [Github](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

```bash
$ cargo install sqlx-cli --no-default-features --features postgres
```

## Migrations

To create a migration:

```bash
$ sqlx migrate add -r <name>
```

Run migrations:

```bash
$ sqlx migrate run
```

Revet migrations:

```bash
$ sqlx migrate revert
```

# Cargo watch

Usage:

```bash
$ cargo watch -x 'run --bin api'
```

# Benchmark

Use [Drill](https://github.com/fcsonline/drill)

```bash
$ drill --benchmark drill.yml --stats --quiet
```

# Documentation

Run:

```bash
$ cargo doc --open --no-deps
```

Run with private items:

```bash
$ cargo doc --open --no-deps --document-private-items
```

# Build Docker image with CNCF Buildpacks

```bash
$ pack build vietnamdevsgroup/northwind-rs -e SQLX_OFFLINE=true -b docker.io/paketocommunity/rust
```

Un-comment section `northwindrs` in `docker-compose.yaml` file, then run:

```bash
$ docker-compose up
```