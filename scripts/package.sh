#!/usr/bin/env bash

TARGET="x86_64-apple-darwin"
BINARY="coop"
DIR="coop-${TARGET}"

mkdir ${DIR}

cp target/release/${BINARY} ${DIR}/
cp -R contrib ${DIR}/contrib

zip -r coop-${TARGET}.zip ${DIR}

rm -rf ${DIR}

shasum -a 256 coop-${TARGET}.zip
