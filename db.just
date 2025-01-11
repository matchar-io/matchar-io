create environment:
  dotenvx run -f .env.{{environment}} -- cargo sqlx database create

drop environment:
  dotenvx run -f .env.{{environment}} -- cargo sqlx database drop

add name:
  cargo sqlx migrate add -r {{name}}

migrate environment:
  dotenvx run -f .env.{{environment}} -- cargo sqlx migrate run

revert environment:
  dotenvx run -f .env.{{environment}} -- cargo sqlx migrate revert

prepare environment:
  dotenvx run -f .env.{{environment}} -- cargo sqlx prepare --workspace
