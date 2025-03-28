FROM debian:bullseye as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    wget \
    git \
    build-essential \
    libpcre3-dev \
    zlib1g-dev \
    libzstd-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*    

# Download and extract nginx source
WORKDIR /src
RUN wget https://nginx.org/download/nginx-1.27.4.tar.gz && \
    tar -xzf nginx-1.27.4.tar.gz

# Clone zstd-nginx-module
RUN git clone https://github.com/tokers/zstd-nginx-module.git

# Compile nginx with zstd module
WORKDIR /src/nginx-1.27.4
RUN ./configure \
    --add-module=../zstd-nginx-module \
    --prefix=/etc/nginx \
    --sbin-path=/usr/sbin/nginx \
    --conf-path=/etc/nginx/nginx.conf \
    --error-log-path=/var/log/nginx/error.log \
    --http-log-path=/var/log/nginx/access.log \
    --pid-path=/var/run/nginx.pid \
    --lock-path=/var/run/nginx.lock \
    --user=nginx \
    --group=nginx \
    && make \
    && make install

# Copy mime.types file - Add this
RUN cp conf/mime.types /etc/nginx/mime.types

# Runtime stage
FROM debian:bullseye-slim

# Install required runtime libraries
RUN apt-get update && apt-get install -y \
    libpcre3 \
    zlib1g \
    libzstd1 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled nginx binary and modules
COPY --from=builder /usr/sbin/nginx /usr/sbin/nginx
# Copy mime.types - Add this line
COPY --from=builder /etc/nginx/mime.types /etc/nginx/mime.types

# Create necessary directories and user
RUN addgroup --system nginx || true \
    && adduser --system --disabled-login --ingroup nginx --no-create-home --home /nonexistent --gecos "nginx user" --shell /bin/false nginx || true \
    && mkdir -p /var/log/nginx \
    && mkdir -p /var/run \
    && chown -R nginx:nginx /var/log/nginx \
    && mkdir -p /usr/share/nginx/assets \
    && chown -R nginx:nginx /usr/share/nginx/assets

CMD ["nginx", "-g", "daemon off;"]