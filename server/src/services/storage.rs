pub async fn read_file(path: &str) -> Result<Vec<u8>, std::io::Error> {
    tokio::fs::read(path).await
}