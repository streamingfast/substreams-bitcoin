# Substreams Bitcoin

Substreams development kit for Bitcoin chains, contains Rust Firehose Block model and helpers.

## Usage

```toml
[package]
name = "substreams-acme"
version = 0.1.2

[lib]
crate-type = ["cdylib"]

[dependencies]
substreams-bitoin = "1.0.0"
```

## Development

We manually keep in sync the rendered Rust Firehose Block models with the actual Protocol Buffer definitions file found in [firehose-bitcoin](https://github.com/streamingfast/firehose-bitcoin/tree/develop/proto) and we commit them to Git.

This means changes to Protobuf files must be manually re-generated and commit, see below for how to do it.

### Regenerate Rust Firehose Block from Protobuf

```
./gen.sh
```

If you struggle with something, reach out to us on Discord and we are going to help you out.
