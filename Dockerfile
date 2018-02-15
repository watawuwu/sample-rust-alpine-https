FROM ekidd/rust-musl-builder:stable AS rust-builder
ADD . .
RUN make release

FROM alpine:3.7
COPY --from=rust-builder /home/rust/src/target/x86_64-unknown-linux-musl/release/gununununu /usr/local/bin
RUN apk add --no-cache ca-certificates && update-ca-certificates
# when you do not use openssl-probe
# ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
#    SSL_CERT_DIR=/etc/ssl/certs
CMD ["gununununu"]
