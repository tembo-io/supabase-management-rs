use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageConfig {
    pub file_size_limit: i64,
    pub features: Features,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Features {
    pub image_transformation: ImageTransformation,
    #[serde(rename = "s3Protocol")]
    pub s3protocol: S3Protocol,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageTransformation {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct S3Protocol {
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub created_at: String,
    pub updated_at: String,
    pub public: bool,
}

impl crate::Client {
    /// Gets project's storage config
    pub async fn get_storage_config(
        &self,
        project_id: &str,
    ) -> Result<StorageConfig, crate::Error> {
        self.get(format_args!("projects/{project_id}/config/storage"))
            .await
    }

    pub async fn list_buckets(&self, project_id: &str) -> Result<Vec<Bucket>, crate::Error> {
        self.get(format_args!("projects/{project_id}/storage/buckets"))
            .await
    }
}
