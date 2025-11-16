pub mod env;
pub mod log;

// 서버 시작 전 설정해야하는 것들
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    env::init_env()?;
    log::init_log();

    Ok(())
}