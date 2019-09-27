
FROM debian:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    g++ \
    curl \
 && rm -rf /var/chache/apt/lists/* \
 &&  curl https://sh.rustup.rs -sSf | sh -s -- -y

ENV PATH=/root/.cargo/bin:$PATH
