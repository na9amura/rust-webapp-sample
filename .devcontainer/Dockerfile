# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.195.0/containers/rust/.devcontainer/base.Dockerfile
# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

USER vscode

# [Optional] Uncomment this section to install additional packages.
RUN sudo apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && sudo apt-get -y install --no-install-recommends postgresql
 
RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres \
    && cargo install sccache

# export RUSTC_WRAPPER=$(which sccache)
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
