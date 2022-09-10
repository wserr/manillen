FROM rust:latest as builder 
RUN apt-get update 
RUN apt-get install -y librust-openssl-sys-dev libssl-dev
WORKDIR /usr/src/myapp 
COPY . . 
RUN cargo install --path . 
 
CMD ["chat"] 