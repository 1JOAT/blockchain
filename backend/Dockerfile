# ---------- Stage 1: Build ----------
FROM rust:1.74 as builder
WORKDIR /app

# Copy the entire backend folder
COPY . .

# Build your Rust backend in release mode
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/backend /app/backend

# Expose the port your backend listens on
EXPOSE 3000

# Run the backend binary
CMD ["./backend"]
