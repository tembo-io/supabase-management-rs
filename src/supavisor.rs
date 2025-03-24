use serde::{Deserialize, Serialize};

use crate::Client;

impl Client {
    pub async fn get_supavisor_details(
        &self,
        project_id: &str,
    ) -> Result<Vec<SupavisorConfig>, crate::Error> {
        self.get(format_args!("projects/{project_id}/config/database/pooler"))
            .await
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DatabaseType {
    Primary,
    ReadReplica,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum PoolMode {
    Transaction,
    Session,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SupavisorConfig {
    pub database_type: DatabaseType,
    pub db_port: i32,
    pub default_pool_size: Option<i32>,
    pub max_client_conn: Option<i32>,
    pub identifier: String,
    pub is_using_scram_auth: bool,
    pub db_user: String,
    pub db_host: String,
    pub db_name: String,
    pub pool_mode: PoolMode,
}
