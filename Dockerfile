FROM rust:slim

RUN apt-get update && apt-get install -y build-essential pkg-config libssl-dev

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

ENTRYPOINT ["cargo", "test", "--test", "cucumber_runner"]

# Default command to run tests without tags
CMD []
