//! # Supabase Management API Client
//!
//! **⚠️ Note: This crate is still a work in progress and not all API endpoints are implemented yet.**
//!
//! This crate provides a client for interacting with the [Supabase Management API](https://supabase.com/docs/reference/api/introduction).
//!
//! It allows management of Supabase projects, including:
//!
//! - **Organization management**: View and manage organizations
//! - **Project operations**: Create, list, retrieve, update, delete, pause, and restore projects
//! - **Project configuration**: Manage database settings, API keys, and network restrictions
//! - **Database management**: Execute queries, manage database branches, and view usage metrics
//! - **Storage management**: Configure buckets, policies, and other storage settings
//! - **Functions management**: Deploy, list and configure edge functions
//! - **Project monitoring**: Check health status and view logs
//! - **SSL enforcement**: Configure custom domains and SSL settings
//! - **Postgres extensions**: Manage available and enabled extensions
//!
//! ## Example
//!
//! ```no_run
//! use supabase_management_rs::Client;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client with your Supabase management API key
//!     let client = Client::new("your-api-key".to_string());
//!
//!     // List all projects
//!     let projects = client.list_projects().await?;
//!
//!     // Get the first project
//!     if let Some(project) = projects.first() {
//!         println!("Project name: {}", project.name);
//!
//!         // Check project health
//!         let health = client.get_project_health(&project.id).await?;
//!         println!("Project health: {:?}", health);
//!
//!         // Execute a query
//!         let results: serde_json::Value = client
//!             .query(&project.id, "SELECT now()")
//!             .await?;
//!         println!("Query result: {:?}", results);
//!
//!         // Pause a project
//!         client.pause_project(&project.id).await?;
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::{fmt, sync::LazyLock};

use serde::{de::DeserializeOwned, Serialize};

/// Module for generating access tokens for Supabase projects
pub mod auth;
/// Module for managing Postgres configuration settings of a Supabase project.
pub mod postgres_configs;
/// Module for managing Supabase projects, including creation, deletion, and configuration.
pub mod project;
/// Module for listing and managing storage settings in Supabase projects.
pub mod storage;

pub(crate) static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);

/// A client to interact with the Supabase Management API.
#[derive(Clone)]
pub struct Client {
    api_key: String,
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Client").finish()
    }
}

impl Client {
    /// Creates a new client with the given API key.
    ///
    /// See [the docs](https://supabase.com/docs/reference/api/introduction) to learn how to obtain an API key.
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    /// [Beta endpoint] Executes a Postgres query using the Supabase Management API.
    ///
    /// ```no_run
    /// # use serde::Deserialize;
    /// # async fn run_query() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = supabase_management_rs::Client::new("dummy".to_string());
    ///
    /// #     let project_id = "dummy";
    ///
    /// #[derive(Deserialize, PartialEq, Debug)]
    /// struct Row {
    ///     id: i32,
    ///     hash_value: String,
    /// }
    ///
    /// let rows: Vec<Row> = client
    ///     .query(
    ///         project_id,
    ///         "SELECT generate_series(1, 3) AS id, \
    ///         md5(generate_series(1, 3)::text) AS hash_value",
    ///     )
    ///     .await?;
    ///
    /// assert_eq!(
    ///     rows,
    ///     [
    ///         Row {
    ///             id: 1,
    ///             hash_value: "c4ca4238a0b923820dcc509a6f75849b".into(),
    ///         },
    ///         Row {
    ///             id: 2,
    ///             hash_value: "c81e728d9d4c2f636f067f89cc14862c".into(),
    ///         },
    ///         Row {
    ///             id: 3,
    ///             hash_value: "eccbc87e4b5ce2fe28308fd9f2a7baf3".into(),
    ///         },
    ///     ]
    /// );
    /// #     Ok(())
    /// # }
    pub async fn query<T: DeserializeOwned>(
        &self,
        project_id: &str,
        query: &str,
    ) -> Result<T, reqwest::Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            query: &'a str,
        }

        let url = format!("https://api.supabase.com/v1/projects/{project_id}/database/query");

        CLIENT
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&Body { query })
            .send()
            .await?
            .json()
            .await
    }
}
