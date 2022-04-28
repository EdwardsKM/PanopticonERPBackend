pub mod rustls_config {
    use rustls::{Certificate, ClientConfig, PrivateKey, ServerConfig};
    use rustls_pemfile::{certs, pkcs8_private_keys};
    use std::{fs::File, io::BufReader};

    pub fn load_rustls_config() -> rustls::ServerConfig {
        // init server config builder with safe defaults
        let rustlsconfig = ServerConfig::builder()
            .with_safe_default_cipher_suites()
            .with_safe_default_kx_groups()
            .with_safe_default_protocol_versions()
            .unwrap()
            .with_no_client_auth();

        // load TLS key/cert files
        let cert_file = &mut BufReader::new(File::open("./cert.pem").unwrap());
        let private_key_file = &mut BufReader::new(File::open("./key.pem").unwrap());

        // convert files to key/cert objects
        let cert_chain = certs(cert_file)
            .unwrap()
            .into_iter()
            .map(Certificate)
            .collect();
        let mut keys: Vec<PrivateKey> = pkcs8_private_keys(private_key_file)
            .unwrap()
            .into_iter()
            .map(PrivateKey)
            .collect();

        // exit if no keys could be parsed
        if keys.is_empty() {
            eprintln!("Could not locate PKCS 8 private keys.");
            std::process::exit(1);
        }

        rustlsconfig
            .with_single_cert(cert_chain, keys.remove(0))
            .unwrap()
    }

    pub fn _db_tls_connection() {
        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(rustls::RootCertStore::empty())
            .with_no_client_auth();

        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(config);

        let _connect_fut =
            tokio_postgres::connect("sslmode=require host=localhost user=postgres", tls);
    }
}
