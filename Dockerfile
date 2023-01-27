FROM rust:1.67.0 as chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc as runtime
WORKDIR /app
COPY --from=builder /app/target/release/bag_of_holding ./
EXPOSE 5000
EXPOSE 9000
CMD ["./bag_of_holding"]
