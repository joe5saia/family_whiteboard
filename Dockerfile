# Build stage
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Set working directory
WORKDIR /app

# Copy manifest files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY migrations ./migrations

# Build the WASM package
RUN wasm-pack build --target web --release

# Build the Rust binary for database operations
RUN cargo build --release --bin server

# Runtime stage
FROM nginx:alpine

# Install postgresql client
RUN apk add --no-cache postgresql-client

# Copy static files
COPY frontend/index.html /usr/share/nginx/html/
COPY frontend/styles.css /usr/share/nginx/html/
COPY frontend/app.js /usr/share/nginx/html/
COPY frontend/app-wasm.js /usr/share/nginx/html/
COPY frontend/api-client.js /usr/share/nginx/html/

# Copy WASM artifacts
COPY --from=builder /app/pkg /usr/share/nginx/html/pkg/

# Copy nginx configuration
COPY nginx.conf /etc/nginx/nginx.conf

# Create directory for application data
RUN mkdir -p /data && chown nginx:nginx /data

# Expose port
EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]