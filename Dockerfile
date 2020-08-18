FROM rust:1.45-stretch as builder
WORKDIR /usr/src/pe
RUN USER=root cargo new --bin protocoleexquis
WORKDIR ./protocoleexquis
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./diesel.toml ./diesel.toml
COPY ./.env ./.env
COPY ./static ./static
COPY ./migrations ./migrations
COPY ./src ./src
RUN rm ./target/release/deps/server-*
RUN cargo build --release
RUN mkdir -p /usr/src/pe/out
RUN cp ./target/release/server /usr/src/pe/out/
RUN cargo install --path .
EXPOSE 34787
CMD ["server"]


#WORKDIR /usr/src/pe
#COPY . .
#RUN cargo install --path .
#
#EXPOSE 34787

#CMD ["server"]