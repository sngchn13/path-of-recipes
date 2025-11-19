pub fn init_env() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenvy::from_filename(".env.prod");
    let _ = dotenvy::from_filename(".env.local");    //from_filename은 같은 변수가 있을때 기존의 값을 사용함.
    dotenvy::dotenv()?; // 실행파일의 current와 parent에서 .env를 찾음
    Ok(())
}