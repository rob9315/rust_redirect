FROM alpine
COPY target/release/rust_redirect rust_redirect
HEALTHCHECK NONE
CMD rust_redirect
