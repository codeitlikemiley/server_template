# Stage 1: Generate a recipe file for dependencies
FROM rust:1.70 as planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Stage 2: Build our dependencies
FROM rust:1.70 as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Stage 3: Use the main official Rust Docker image as our builder
FROM rust:1.70 as builder
# Secure Docker image by creating non-root user
ENV USER=web
ENV UID=1001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"
COPY . /app
WORKDIR /app
ARG DB_URL=postgresql://localhost:5432/postgres
ENV DB_URL=${DB_URL}
# Copy dependencies so we don't need to rebuild
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

# Install Protobuf
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

# Build the app using cached dependencies
RUN cargo build --release

# Stage 4: Final stage - Reduce the size of the Docker image
FROM gcr.io/distroless/cc-debian11
# Import the non-root user from the builder
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
# Copy the app from the builder
COPY --from=builder /app/target/release/rust-grpc /app/rust-grpc
# Copy reflection descriptor so we can use evans cli
COPY --from=builder  /app/pb/reflection_descriptor.bin /app/pb/reflection_descriptor.bin
WORKDIR /app
# Set the user to non-root
USER web:web
# Start the application
CMD ["./rust-grpc"]
