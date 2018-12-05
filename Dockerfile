FROM rustlang/rust:nightly
ARG K8SR_SERVICE
ENV K8SR_SERVICE ${K8SR_SERVICE}

WORKDIR /usr/src/k8sr
COPY . .

RUN wget http://packages.couchbase.com/ubuntu/couchbase.key && \
    apt-key add couchbase.key && \
    echo "deb http://packages.couchbase.com/ubuntu stretch stretch/main" > /etc/apt/sources.list.d/couchbase.list && \
    apt-get update && \
    apt-get install -y libcouchbase2-core libcouchbase-dev

RUN cargo install --bin ${K8SR_SERVICE} --path .
EXPOSE 8000

CMD ${K8SR_SERVICE}
