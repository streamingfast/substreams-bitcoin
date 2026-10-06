# Change log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [3.0.0](https://github.com/streamingfast/substreams-bitcoin/releases/tag/v3.0.0)

### Changed

- **Breaking**: protobuf encoding and decoding moved from [prost](https://github.com/tokio-rs/prost)
  to [buffa](https://github.com/anthropics/buffa), so this crate now requires `substreams` 0.8.0 or
  above. `prost`, `prost-types` and `prost-build` are no longer dependencies. See
  [Migrating from prost to buffa](https://github.com/streamingfast/substreams/blob/develop/docs/references/migrating-to-buffa.md).

  The block model's two optional message fields, `Vin::script_sig` and `Vout::script_pub_key`, are
  now `MessageField<T>` rather than `Option<T>`. `MessageField` derefs to a default instance, so
  `vin.script_sig.asm` reads directly without unwrapping. Every type name is unchanged.

- The block model is generated from the `buf.build/streamingfast/firehose-bitcoin` module declared in
  `buf.gen.yaml`, so `buf generate` regenerates it and `gen.sh` is removed.

- The toolchain is pinned to 1.93 and `rust-version` is raised from 1.60 to 1.83, the floor set by
  `substreams` 0.8.0.

## [2.0.0]

* Bump substreams to 0.6, prost to 0.13.3, see [update notes](https://github.com/streamingfast/substreams-rs/releases/tag/v0.6.0)

## [Unreleased]

* StreamingFast Firehose Block generated Rust code is now included in this library directly.
