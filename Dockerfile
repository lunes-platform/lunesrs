FROM rust

# install rust
RUN apt update -y && \
    curl -sL https://deb.nodesource.com/setup_14.x | bash - && \
    apt install nodejs -y

# install rust dependencies
RUN cargo install wasm-pack
RUN cargo install cargo-watch
RUN cargo install cargo-x
RUN rustup component add rustfmt