FROM alpine
COPY target/release/rust_redirect rust_redirect
EXPOSE 80/tcp
CMD rust_redirect
