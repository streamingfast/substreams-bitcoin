#!/bin/bash

set -e

BTC_SPKG="${BTC_SPKG:-https://github.com/streamingfast/firehose-bitcoin/releases/download/v1.0.0-rc.1/bitcoin-v1.0.0.spkg}"
input="$BTC_SPKG"#format=bin

echo "Generating Bitcoin Protobuf using $input"
buf generate "$input" --exclude-path sf/substreams/v1,sf/substreams/rpc,google/,sf/substreams/sink,sf/substreams
