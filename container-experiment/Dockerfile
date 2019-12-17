FROM rust:1.39

WORKDIR /usr/src/container-experiment
COPY . .

RUN cargo install --path .

CMD ["container-experiment"]
