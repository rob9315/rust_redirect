FROM alpine
COPY target/release/rust_redirect rust_redirect
EXPOSE 443
CMD rust_redirect
