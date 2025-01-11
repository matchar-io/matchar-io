mod env 'env.just'
mod db 'db.just'

build environment:
  dotenvx run -f .env.{{environment}} -- cargo build

check environment:
  dotenvx run -f .env.{{environment}} -- cargo check

run environment package:
  dotenvx run -f .env.{{environment}} -- cargo run --package {{package}}

test environment *package:
  dotenvx run -f .env.{{environment}} -- cargo test {{package}}
