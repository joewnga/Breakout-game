FROM rust:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    cmake \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -ms /bin/bash devuser

WORKDIR /workspace

USER devuser

COPY . .

RUN cargo install cargo-watch

EXPOSE 8000

CMD ["cargo", "run"]
