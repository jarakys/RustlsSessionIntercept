mod CertificateResolver;
mod NoVerifier;

use std::io::{Read};
use rustls;
use rustls::{RootCertStore};
use rustls::client::danger::{ServerCertVerifier};
use rustls::pki_types::{CertificateDer};

fn main() {
    let mut root_store = RootCertStore::empty();
    let file = std::fs::read("./certs/ca.der").unwrap();
    root_store.add(CertificateDer::from_slice(file.as_slice())).unwrap();
    let resolver = CertificateResolver::CertificateResolver::new("google.com".to_string(), 443);
    let cert = resolver.resolve(root_store);
    println!("{:?}", cert);
}