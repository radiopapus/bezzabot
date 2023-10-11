FROM rust as base

ENV DEBIAN_FRONTEND=noninteractive
ENV TARGET=arm-linux-gnueabihf
ENV PKG_CONFIG_PATH=/usr/lib/$TARGET/pkgconfig
ENV AR=${TARGET}-ar CC=${TARGET}-gcc

# Install deps and remove libssl-dev
RUN apt update &&  \
    apt install -y tini gcc-${TARGET} ca-certificates curl gnupg &&  \
    apt remove -y libssl-dev && \
    apt update && \
    mkdir -p /etc/apt/keyrings && \
    curl -fsSL https://deb.nodesource.com/gpgkey/nodesource-repo.gpg.key | gpg --dearmor -o /etc/apt/keyrings/nodesource.gpg && \
    apt update && apt install -y nodejs  && apt autoremove -y && apt clean -y

FROM base AS source
# build openssl from source - need old version for armv6
# why not cross? https://github.com/cross-rs/cross/wiki/Additional-External-Dependency-Issues
# https://github.com/sfackler/rust-openssl/blob/master/.github/workflows/ci.yml
# https://github.com/frida/frida/issues/535
ENV OPENSSL_VERSION=1.0.1 OPENSSL_TMP_PATH=/tmp/build
ENV OPENSSL_URL=https://openssl.org/source/old/${OPENSSL_VERSION}/openssl-${OPENSSL_VERSION}u.tar.gz

RUN mkdir -p ${OPENSSL_TMP_PATH} && cd ${OPENSSL_TMP_PATH} &&  \
    curl -fsL ${OPENSSL_URL} | tar --strip-components=1 -xzf - &&  \
    chmod +x Configure && ${OPENSSL_TMP_PATH}/Configure --libdir=lib linux-armv4 -fPIC -g no-shared &&  \
    make && make install_sw

FROM source AS target

ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_LINKER=$CC
ENV CARGO_TARGET_ARM_UNKNOWN_LINUX_GNUEABIHF_AR=$AR
ENV RUST_TARGET=arm-unknown-linux-gnueabihf

# Enable ARMv6 for Rust
RUN rustup target add ${RUST_TARGET}

WORKDIR /bezzabot

# Setup entrypoint
COPY docker/scripts/entrypoint-helpers.sh docker/scripts/entrypoint-helpers.sh
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["tini", "--", "docker-entrypoint.sh"]
