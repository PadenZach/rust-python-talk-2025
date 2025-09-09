FROM mcr.microsoft.com/devcontainers/python:3.12-bookworm
# Add postgres apt repo that has newer versions of postgres available.
RUN apt update

RUN sudo apt install ripgrep hyperfine

ARG NODE_VERSION="lts/*"
RUN su vscode -c "umask 0002 && . /usr/local/share/nvm/nvm.sh && nvm install ${NODE_VERSION} && nvm alias default ${NODE_VERSION}"

# install uv
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV UV_PYTHON_PREFERENCE="managed"
ENV UV_LINK_MODE="copy"
ENV UV_CACHE_DIR=/.uv_cache

ARG USERNAME=vscode

# Create directories in non-user locations
RUN mkdir -p /commandhistory \
    && mkdir -p /.uv_cache \
    && touch /commandhistory/.zsh_history \
    && chmod -R 777 /commandhistory \
    && chmod -R 777 /.uv_cache
