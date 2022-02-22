FROM rust:latest


WORKDIR /yew-app

COPY ./server .

RUN ls

RUN pwd


RUN cargo build --release

RUN cargo install --path .

EXPOSE 8080

CMD server