# Base image for building and running
FROM rust:1.83

# Install Trunk
RUN cargo install trunk
RUN cargo install --git https://github.com/camilomcatasus/trunk_repl
RUN rustup update
RUN rustup target add wasm32-unknown-unknown


# Set the working directory
WORKDIR /app

# Copy the project files
COPY . .

# Build the project
RUN trunk build --release --verbose

# Expose the desired port
EXPOSE 8080

# Run Trunk's server
CMD ["trunk", "serve", "--release", "--port", "8080", "--address", "0.0.0.0"]

