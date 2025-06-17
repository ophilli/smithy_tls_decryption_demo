# Smithy TLS Decryption Demo

This demo demonstrates how to make a custom client so that TLS decryption can be enabled.

Use `wireshark -o "tls.keylog_file:<path>"` to start recording traffic and `SSLKEYLOGFILE=<path> cargo run` to generate some traffic & save the key.

See [wireshark's tls decryption](https://wiki.wireshark.org/TLS#tls-decryption) docs and [rustls' ClientConfig](https://docs.rs/rustls/latest/rustls/client/struct.ClientConfig.html#structfield.key_log) docs for more info.
