use serde::{Deserialize, Serialize};

use crate::Client;

impl Client {
    /// List all projects this user has created
    pub async fn list_projects(&self) -> Result<Vec<Project>, reqwest::Error> {
        let url = "https://api.supabase.com/v1/projects";

        self.client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?
            .json()
            .await
    }

    /// Pause an active project. Returns 400 if the project is already paused.
    pub async fn pause_project(&self, project_id: &str) -> Result<(), reqwest::Error> {
        let url = format!("https://api.supabase.com/v1/projects/{}/pause", project_id);

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        response.error_for_status().map(|_| {})
    }

    /// Restore a paused/inactive project.
    pub async fn restore_project(&self, project_id: &str) -> Result<(), reqwest::Error> {
        let url = format!(
            "https://api.supabase.com/v1/projects/{}/restore",
            project_id
        );

        let response = self
            .client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        response.error_for_status().map(|_| {})
    }

    /// Gets project's service health status.
    pub async fn get_project_health(
        &self,
        project_id: &str,
    ) -> Result<Vec<ServiceHealth>, reqwest::Error> {
        let url = format!(
            "https://api.supabase.com/v1/projects/{}/health?services=auth,db,pooler,storage",
            project_id
        );

        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        response.error_for_status()?.json().await
    }
}

/// Represents a Supabase project.
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub region: String,
    pub created_at: String,
    pub status: Status,
    pub database: Database,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Inactive,
    ActiveHealthy,
    ActiveUnhealthy,
    ComingUp,
    Unknown,
    GoingDown,
    InitFailed,
    Removed,
    Restoring,
    Upgrading,
    Pausing,
    RestoreFailed,
    Restarting,
    PauseFailed,
    Resizing,
}

/// Represents a Supabase database.
#[derive(Serialize, Deserialize, Debug)]
pub struct Database {
    pub host: String,
    pub version: String,
    pub postgres_engine: String,
    pub release_channel: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServiceHealth {
    pub name: String,
    pub healthy: bool,
    pub status: Status,
}
