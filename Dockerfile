FROM rust

RUN apt-get update -y && \
    curl -sL https://deb.nodesource.com/setup_14.x | bash - && \
    apt-get install nodejs -y && \
    cargo install wasm-pack && \
    rustup target add wasm32-unknown-unknown && \
    rustup component add rustfmt
