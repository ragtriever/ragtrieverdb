use aws_sdk_s3::*;
use std::fs::File;
use std::io::Read;

pub struct S3Wrapper {
    client: Client,
    bucket: String,
}

impl S3Wrapper {
    pub async fn new(bucket: &str, region: &str) -> Result<Self, Error> {
        let config = aws_config::load_from_env().await;
        // let config = aws_config::from_env().region(region.into()).load().await;
        let client = Client::new(&config);
        Ok(S3Wrapper {
            client,
            bucket: bucket.to_string(),
        })
    }

    pub async fn upload_file(
        &self,
        namespace: &str,
        id_file: &str,
        file_path: &str,
    ) -> Result<(), Error> {
        let key = format!("{}/{}", namespace.trim_matches('/'), id_file);

        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .body(buffer.into())
            .send()
            .await?;

        Ok(())
    }
}
