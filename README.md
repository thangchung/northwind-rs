# northwind-rs

The Northwind application powered by Rust (actix-web, sqlx, jwt...)

Generated from [`actix-sqlx-boilerplate`](https://github.com/fabienbellanger/actix-sqlx-boilerplate)

## Give a star ⭐

If you're using this repository for your samples, workshop, your project or whatever, please give a star ⭐. Thank you very much :+1:

# Up and running

## Run server

```bash
$ docker-compose up
$ cargo run --bin northwind-actix
```

Then play around with `restclient.http`

# SQLx

sqlx repository: [Github](https://github.com/launchbadge/sqlx)
## sqlx-cli

sqlx-cli repository: [Github](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli)

```bash
$ cargo install sqlx-cli --no-default-features --features postgres
```

### Offline mode

On windows 10

Make sure that you have `docker-compose up` running with at least `postgresdb`

```bash
$ cargo clean
$ cmd /c "set SQLX_OFFLINE=true && cargo sqlx prepare -- --manifest-path apps/actix/Cargo.toml --bin northwind-actix"
```

Make sure, we have `sqlx-data.json` updated. Then, we can build the buildpack below

The issue couldn't run `cargo sqlx` cli successfully at https://github.com/launchbadge/sqlx/issues/788

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
$ pack build vietnamdevsgroup/northwind-rs -e SQLX_OFFLINE=true -e BP_CARGO_INSTALL_ARGS="--path=./apps/actix" 
-b docker.io/paketocommunity/rust
```

Un-comment section `northwindrs` in `docker-compose.yaml` file, then run:

```bash
$ docker-compose up
```

# Troubleshooting 

## Inspect docker image

```bash
$ docker run -it --entrypoint /bin/bash vietnamdevsgroup/northwind-rs
```

## Dive to see what inside docker image

```bash
$ docker pull wagoodman/dive:latest
$ docker run --rm -it -v /var/run/docker.sock:/var/run/docker.sock wagoodman/dive:latest vietnamdevsgroup/northwind-rs
```