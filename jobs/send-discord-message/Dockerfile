FROM rust:1.67

WORKDIR /app

COPY . .

RUN cargo install --path .

CMD send-discord-message