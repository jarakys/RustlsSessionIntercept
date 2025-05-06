use std::net::TcpStream;
use std::sync::Arc;
use rustls::{ClientConfig, ClientConnection, RootCertStore, Stream};
use rustls::pki_types::{ServerName};
use crate::NoVerifier::NoVerifier;

pub struct CertificateResolver {
    host: String,
    port: u16,
}

impl CertificateResolver {
    pub fn new(host: String, port: u16) -> CertificateResolver {
        CertificateResolver {
            host,
            port
        }
    }

    //Return certificate in DerFormat
    pub fn resolve(&self, root_store: RootCertStore) -> Option<Vec<u8>> {
        let domain = ServerName::try_from(self.host.clone()).unwrap();
        let addr = format!("{}:{}", self.host, self.port);
        let mut config = ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();
        config.dangerous().set_certificate_verifier(Arc::new(NoVerifier));

        let mut conn = ClientConnection::new(Arc::new(config), domain).unwrap();
        let mut sock = TcpStream::connect(addr).unwrap();
        sock.set_nonblocking(false).unwrap();

        let mut stream = Stream::new(&mut conn, &mut sock);

        while stream.conn.is_handshaking() {
            if let Ok(ok) = stream.conn.complete_io(&mut stream.sock) {
                println!("Certificate received!");
            } else if let Err(e) = stream.conn.complete_io(&mut stream.sock) {
                println!("Error: {}", e);
            }
        }
        let cert = stream.conn.peer_certificates()?.first()?.to_vec();
        Some(cert)
    }
}