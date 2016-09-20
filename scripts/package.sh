#!/usr/bin/env bash

TARGET="x86_64-apple-darwin"
BINARY="coop"

cp target/release/${BINARY} ${BINARY}
zip coop-${TARGET}.zip ${BINARY}
rm ${BINARY}

shasum -a 256 coop-${TARGET}.zip