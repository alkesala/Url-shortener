FROM rust:1.70 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /usr/src/app/target/release/url-shortener /usr/local/bin/
EXPOSE 8080
CMD ["url-shortener"]