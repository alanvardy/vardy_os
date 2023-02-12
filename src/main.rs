use std::net::TcpListener;

use vardy_os::run;

const PROD_ADDR: &str = "0.0.0.0:8080";
const DEV_ADDR: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = if is_prod() { PROD_ADDR } else { DEV_ADDR };
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener)?.await
}

fn is_prod() -> bool {
    let env = std::env::var("ENV");
    env == Ok(String::from("prod"))
}
