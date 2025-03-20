use serde::{Deserialize, Serialize};

use crate::CLIENT;

/// Represents the configuration settings for a Postgres database.
///
/// ```
/// # use supabase_management_rs::postgres_configs::PostgresConfig;
///
/// let mut postgres_config = PostgresConfig::default();
/// postgres_config.max_parallel_workers = Some(128);
///
/// // You can now update your instance's Postgres configuration using Client::set_postgres_config
/// ```
#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct PostgresConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_cache_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_decoding_work_mem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_work_mem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_activity_query_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_locks_per_transaction: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_maintenance_workers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_workers: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_parallel_workers_per_gather: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_replication_slots: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_slot_wal_keep_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_standby_archive_delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_standby_streaming_delay: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wal_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_wal_senders: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_worker_processes: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_buffers: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_commit_timestamp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_keep_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wal_sender_timeout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_mem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_replication_role: Option<SessionReplicationRole>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SessionReplicationRole {
    Origin,
    Replica,
    Local,
}

impl crate::Client {
    /// Get the Postgres configs for a project
    pub async fn get_postgres_config(
        &self,
        project_id: &str,
    ) -> Result<PostgresConfig, reqwest::Error> {
        let url =
            format!("https://api.supabase.com/v1/projects/{project_id}/config/database/postgres");

        CLIENT
            .get(&url)
            .bearer_auth(&self.api_key)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    pub async fn set_postgres_config(
        &self,
        project_id: &str,
        config: &PostgresConfig,
    ) -> Result<PostgresConfig, reqwest::Error> {
        let url =
            format!("https://api.supabase.com/v1/projects/{project_id}/config/database/postgres");

        CLIENT
            .put(&url)
            .bearer_auth(&self.api_key)
            .json(&config)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::PostgresConfig;

    #[test]
    fn deserializes_pg_config() {
        let json_data = r#"
        {
            "effective_cache_size": "4GB",
            "logical_decoding_work_mem": "64MB",
            "maintenance_work_mem": "256MB",
            "track_activity_query_size": "2048",
            "max_connections": 100,
            "max_locks_per_transaction": 128,
            "max_parallel_maintenance_workers": 2,
            "max_parallel_workers": 4,
            "max_parallel_workers_per_gather": 2,
            "max_replication_slots": 10,
            "max_slot_wal_keep_size": "1GB",
            "max_standby_archive_delay": "30s",
            "max_standby_streaming_delay": "30s",
            "max_wal_size": "2GB",
            "max_wal_senders": 5,
            "max_worker_processes": 8,
            "shared_buffers": "2GB",
            "statement_timeout": "60s",
            "track_commit_timestamp": true,
            "wal_keep_size": "512MB",
            "wal_sender_timeout": "60s",
            "work_mem": "4MB",
            "session_replication_role": "replica"
        }
        "#;

        let _config: PostgresConfig =
            serde_json::from_str(json_data).expect("JSON was not well-formatted");
    }
}
