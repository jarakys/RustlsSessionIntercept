# rustls-mitm-cert-sniffer

A small Rust utility that connects to a remote TLS server and extracts its certificate in DER format using [rustls](https://github.com/rustls/rustls).

This serves as a minimal example for intercepting TLS connections and can be extended for man-in-the-middle (MITM) use cases such as HTTPS traffic decryption in custom firewalls or proxies.

## ⚠️ Disclaimer

This project is for **educational and research purposes only**. Do **not** use it to intercept traffic you don't own or have explicit permission to inspect. Unauthorized use may violate local laws.

## Features

- Establish a TLS connection to any host and port
- Extract the server’s certificate in DER format
- Output or save the certificate for further analysis or re-signing
