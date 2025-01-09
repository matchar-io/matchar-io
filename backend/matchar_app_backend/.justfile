mod env 'env.just'

run environment:
  dotenvx run -f .env.{{environment}} -- cargo run
