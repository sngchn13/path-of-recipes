pub fn init_env() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::from_filename(".env.prod");
    let _ = dotenvy::from_filename(".env.local");
    dotenvy::dotenv()?;
    Ok(())
}