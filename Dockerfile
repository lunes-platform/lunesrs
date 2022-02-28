FROM rust


# install dependencies
RUN apt update -y && apt install zsh curl -y
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# install rust dependencies
RUN cargo install wasm-pack
RUN cargo install cargo-watch
RUN cargo install cargo-x
