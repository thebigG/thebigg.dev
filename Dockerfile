FROM rust:1.64
RUN useradd -ms /bin/bash lgomez
USER lgomez
WORKDIR /home/lgomez
COPY --chown=lgomez:lgomez . /home/lgomez
RUN rustup component add rustfmt
RUN rustfmt --check /home/lgomez/thebigg_dev/src/*.rs
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cd /home/lgomez/thebigg_dev && cargo clean && cargo build --release

CMD cd /home/lgomez/thebigg_dev && trunk serve --address "0.0.0.0"
