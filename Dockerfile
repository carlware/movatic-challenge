FROM rust:1.81-bullseye as build

WORKDIR /app
COPY ./ ./
RUN cargo build --release

FROM rust:1.81-bullseye

COPY --from=build /app/target/release/movatic .
CMD ["./movatic"]
