FROM rust


# install os dependencies
RUN apt update -y && apt upgrade -y
RUN apt install build-essential zsh git curl npm -y
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# install rust dependencies
# for build with ARM64
# RUN cargo install wasm-pack
# for build with AMD64
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh


# docker buildx build \
#     --push \
#     --no-cache \
#     --file .devcontainer/dev/Dockerfile \
#     --platform linux/amd64,linux/arm64/v8 \
#     -t lunesplatform/lunesrs:dev .
