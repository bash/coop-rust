#!/usr/bin/env bash

TARGET="x86_64-apple-darwin"
BINARY="coop"
DIR="coop-${TARGET}"

mkdir ${DIR}

cp target/release/${BINARY} ${DIR}/
cp scripts/bash_completion.sh ${DIR}/

zip -r coop-${TARGET}.zip ${DIR}

rm -rf ${DIR}

shasum -a 256 coop-${TARGET}.zip