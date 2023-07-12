FROM lukemathwalker/cargo-chef:latest-rust-1.70.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .

RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

RUN cargo build --release --bin actix-web-cloud-run
RUN strip target/release/actix-web-cloud-run

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/actix-web-cloud-run actix-web-cloud-run
ENTRYPOINT ["./actix-web-cloud-run"]
