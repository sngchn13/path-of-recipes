use tracing::info;

use crate::config::{env, log};

// 서버 시작 전 설정해야하는 것들
pub fn startup() -> Result<(), Box<dyn std::error::Error>> {
    env::init_env()?;
    log::init_log();

    Ok(())
}

pub fn host_and_port() -> (String, String) {
    let host = std::env::var("APP_HOST").expect("APP_HOST not found");
    let port = std::env::var("APP_PORT").expect("APP_PORT not found");
    info!("ip={}, port= {}", &host, &port);

    (host, port)
}