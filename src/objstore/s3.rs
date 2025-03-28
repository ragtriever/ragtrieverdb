use aws_sdk_s3::{Client, Error};
use std::fs::File;
use std::io::Read;

/// A simple wrapper around the AWS S3 client.
pub struct S3Wrapper {
    client: Client,
    bucket: String,
}

impl S3Wrapper {
    /// Create a new S3Wrapper instance.
    ///
    /// # Arguments
    ///
    /// * `bucket` - The name of the S3 bucket.
    /// * `region` - The AWS region where your bucket is located.
    pub async fn new(bucket: &str, region: &str) -> Result<Self, Error> {
        // Load AWS configuration from the environment (credentials, region, etc.)
        let config = aws_config::from_env().region(region.into()).load().await;
        let client = Client::new(&config);
        Ok(S3Wrapper {
            client,
            bucket: bucket.to_string(),
        })
    }

    /// Upload a file to S3 under the provided namespace.
    ///
    /// The object key will be constructed as: `<namespace>/<id_file>`
    ///
    /// # Arguments
    ///
    /// * `namespace` - The namespace (like a directory).
    /// * `id_file` - The filename (or ID) to use for the S3 object.
    /// * `file_path` - Local path of the file to be uploaded.
    pub async fn upload_file(
        &self,
        namespace: &str,
        id_file: &str,
        file_path: &str,
    ) -> Result<(), Error> {
        // Build the S3 key by combining the namespace and id_file.
        let key = format!("{}/{}", namespace.trim_matches('/'), id_file);

        // Read the entire file into a buffer.
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).expect("Failed to read file");

        // Upload the file to the specified S3 bucket.
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

// Optional: Add tests or example usage.
// For instance, you could write a simple main function (requires Tokio runtime):
//
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let s3 = S3Wrapper::new("your-bucket-name", "us-west-2").await?;
//     s3.upload_file("namespace", "example.bin", "./path/to/example.bin").await?;
//     println!("File uploaded successfully!");
//     Ok(())
// }
