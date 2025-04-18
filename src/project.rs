use serde::{Deserialize, Serialize};

use crate::Client;

impl Client {
    /// List all projects this user has created
    pub async fn list_projects(&self) -> Result<Vec<Project>, crate::error::Error> {
        self.get("projects".into()).await
    }

    /// Pause an active project. Returns 400 if the project is already paused.
    pub async fn pause_project(&self, project_id: &str) -> Result<(), crate::Error> {
        self.post(format!("projects/{}/pause", project_id), None::<()>)
            .await
    }

    /// Restore a paused/inactive project.
    pub async fn restore_project(&self, project_id: &str) -> Result<(), crate::Error> {
        self.post(format!("projects/{}/restore", project_id), None::<()>)
            .await
    }

    /// Gets project's service health status.
    pub async fn get_project_health(
        &self,
        project_id: &str,
    ) -> Result<Vec<ServiceHealth>, crate::Error> {
        self.get(format!(
            "projects/{}/health?services=auth,db,pooler,storage",
            project_id
        ))
        .await
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
