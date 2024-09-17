ARG APP_NAME=docker-rust-actix

FROM rust:1.75

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

# Configure rocket to listen on all interfaces.
ENV ROCKET_ADDRESS=0.0.0.0

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
CMD ["docker-rust-actix"]