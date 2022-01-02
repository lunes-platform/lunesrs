FROM rust

RUN apt-get update -y 
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install nodejs -y
RUN cargo install wasm-pack
