# Sluice Tokio fork

Asynchronous byte buffers and pipes for concurrent I/O programming.

[![Crates.io](https://img.shields.io/crates/v/sluice.svg)](https://crates.io/crates/sluice)
[![Documentation](https://docs.rs/sluice/badge.svg)](https://docs.rs/sluice)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Fork notes

This is a fork of Sluice replacing futures AsyncRead and AsyncWrite implementations with
tokio's AsyncRead and AsyncWrite for tokio runtime compatability.

## [Documentation]

Check the [documentation] for up-to-date usage and examples.

## License

This library is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.


[Documentation]: https://docs.rs/sluice
