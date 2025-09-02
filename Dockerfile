FROM rust:latest

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install SpacetimeDB (latest version - should be 1.2.0)
RUN curl -sSf https://install.spacetimedb.com | sh -s -- -y && \
    echo 'export PATH="/root/.local/bin:$PATH"' >> /root/.bashrc

# Set the path for this session
ENV PATH="/root/.local/bin:$PATH"

# Set working directory
WORKDIR /app

# Copy the world-simulation project
COPY world-simulation/ /app/world-simulation/

# Build the SpacetimeDB module
WORKDIR /app/world-simulation
RUN cargo build --release

# Expose SpacetimeDB default ports
EXPOSE 3001

# Create an entrypoint script
RUN echo '#!/bin/bash\n\
spacetime start --listen-addr 0.0.0.0:3001 &\n\
sleep 5\n\
cd /app/world-simulation\n\
spacetime publish --server http://127.0.0.1:3001 worldsim || true\n\
tail -f /dev/null' > /entrypoint.sh && chmod +x /entrypoint.sh

# Start SpacetimeDB server
CMD ["/entrypoint.sh"]