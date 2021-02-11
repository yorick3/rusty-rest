FROM rust:1.31

WORKDIR /usr/src/rusty-rest
COPY . .

#SET RUST TO NIGHTLY
RUN rustup update nightly; rustup default nightly;

RUN cargo install --path .

CMD ["rusty_rest"]