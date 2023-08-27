# start runner
FROM archlinux:base as runner

RUN pacman -Syyu --noconfirm
RUN pacman -Scc --noconfirm
# end runner

# start builder
FROM runner as builder

RUN pacman -Sy --noconfirm \
  base-devel \
  clang

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

WORKDIR /app

COPY . /app

RUN ~/.cargo/bin/cargo build --release
# end builder

# start runner
FROM runner as app

WORKDIR /app

COPY --from=builder /app/target/release/apcupsd-mqtt-exporter /app/apcupsd-mqtt-exporter

CMD ["./apcupsd-mqtt-exporter"]
# end runner