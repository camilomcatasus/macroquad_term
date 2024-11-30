# Base image for building and running
FROM rust:latest

# Install Trunk
RUN cargo install trunk

# Set the working directory
WORKDIR /app

# Copy the project files
COPY . .

# Build the project
RUN trunk build --release

# Expose the desired port
EXPOSE 8080

# Run Trunk's server
CMD ["trunk", "serve", "--release", "--port", "8080", "--address", "0.0.0.0"]

