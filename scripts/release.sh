#!/bin/sh

TARGET="x86_64-apple-darwin"
BINARY="coop"

cargo build --release --target ${TARGET}

cp target/release/${BINARY} ${BINARY}
zip coop-${TARGET}.zip ${BINARY}
rm ${BINARY}

shasum -a 256 coop-${TARGET}.zip