FROM rust:1.72

WORKDIR /usr/src/figure-backend

COPY . .

RUN cargo install --path .

CMD ["figure-backend"]