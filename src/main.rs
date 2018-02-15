extern crate reqwest;
extern crate openssl_probe;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();
    let uri = "https://httpbin.org/uuid";
    let uuid = match reqwest::get(uri).map(|mut r| r.text().unwrap()) {
        Ok(res) => res,
        Err(err) => err.to_string()
    };

    println!("{}", uuid);
}
