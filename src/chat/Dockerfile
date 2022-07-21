FROM rust:1.62.1-slim as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/chat /usr/local/bin/chat
CMD ["chat"]
EXPOSE 8080

# For debugging purposes, keeps container running when started
#ENTRYPOINT ["tail", "-f", "/dev/null"]