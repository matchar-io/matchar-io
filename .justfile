mod env 'env.just'

build environment:
  dotenvx run -f .env.{{environment}} -- cargo build

run environment package:
  dotenvx run -f .env.{{environment}} -- cargo run --package {{package}}

test environment *package:
  dotenvx run -f .env.{{environment}} -- cargo test {{package}}
