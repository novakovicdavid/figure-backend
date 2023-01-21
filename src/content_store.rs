use std::env;
use async_trait::async_trait;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_sdk_s3::{Client, Config, Credentials};
use aws_sdk_s3::Region;
use aws_sdk_s3::types::ByteStream;
use bytes::Bytes;
use crate::server_errors::ServerError;

#[async_trait]
pub trait ContentStoreFns: Sync + Send {
    async fn upload_object(&self, name: &str, bytes: Bytes) -> Result<(), ServerError<String>>;
    fn get_base_url(&self) -> String;
}

pub type ContentStore = Box<dyn ContentStoreFns>;

pub struct S3Storage {
    client: Client,
    bucket: String,
    base_storage_url: String
}

#[async_trait]
impl ContentStoreFns for S3Storage {
    async fn upload_object(&self, name: &str, bytes: Bytes) -> Result<(), ServerError<String>> {
        self.client.put_object()
            .bucket(&self.bucket)
            .key(name)
            .content_type("image/jpeg")
            .body(ByteStream::from(bytes))
            .send().await
            .map(|_| ())
            .map_err(|e| ServerError::InternalError(e.to_string()))
    }

    fn get_base_url(&self) -> String {
        self.base_storage_url.clone()
    }
}

impl S3Storage {
    pub fn new_store(key_id: String, app_key: String, s3_region: String, bucket_endpoint: String, base_storage_url: String, bucket: String) -> ContentStore {
        let provider_name = "my-creds";
        let creds = Credentials::new(key_id, app_key, None, None, provider_name);

        let config = Config::builder()
            .region(Region::new(s3_region))
            .endpoint_url(bucket_endpoint)
            .credentials_provider(SharedCredentialsProvider::new(creds))
            .build();

        let client = Client::from_conf(config);

        Box::new(Self {
            client,
            bucket,
            base_storage_url
        })
    }
}