FROM rust:latest AS builder
LABEL authors="xorsense"
COPY . /app
WORKDIR /app
RUN ["cargo", "build", "--release"]

FROM ubuntu:latest AS runner
COPY --from=builder /app/target/release/mnstr_api /usr/local/bin/

ENTRYPOINT ["mnstr_api"]