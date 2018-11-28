FROM rustlang/rust:nightly

WORKDIR /usr/src/k8sr0

COPY . .

RUN wget http://packages.couchbase.com/ubuntu/couchbase.key && \
    apt-key add couchbase.key && \
    echo "deb http://packages.couchbase.com/ubuntu stretch stretch/main" > /etc/apt/sources.list.d/couchbase.list && \
    apt-get update && \
    apt-get install -y libcouchbase2-core libcouchbase-dev

RUN cargo install --path .
EXPOSE 8000

CMD ["k8sr0"]
