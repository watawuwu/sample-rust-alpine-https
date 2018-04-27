extern crate reqwest;
extern crate openssl_probe;

use std::result::Result;

fn main() {
    match uuid() {
        Ok(uuid) => println!("{}", uuid),
        Err(err) => eprintln!("{:?}", err),
    }
}

fn uuid() -> Result<String, reqwest::Error> {
    openssl_probe::init_ssl_cert_env_vars();
    let uri = "https://httpbin.org/uuid";
    let result = reqwest::get(uri).map(|mut r| r.text().unwrap())?;
    Ok(result)
}

#[cfg(test)]
mod tests {

    use uuid;

    #[test]
    fn test_run() {
        let uuid = uuid();
        assert!(uuid.is_ok());
    }
}
