# Use an official Rust runtime as a parent image
FROM rust:1.69

# Set the current working directory inside the container
WORKDIR /usr/src/myapp

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build the application
RUN cargo build --release

# Specify the command to run on container start
CMD ["./target/release/rust_crud_api"]