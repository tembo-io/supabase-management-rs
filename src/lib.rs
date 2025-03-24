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

/// Module for generating access tokens for Supabase projects
mod auth;
mod error;
/// Module for managing Postgres configuration settings of a Supabase project
mod postgres_configs;
/// Module for managing Supabase projects, including creation, deletion, and configuration
mod project;
/// Module for executing SQL queries on Supabase projects
mod query;
/// Module for listing and managing storage settings in Supabase projects
mod storage;
/// Get supavisor details
mod supavisor;

pub use auth::*;
pub use error::Error;
pub use postgres_configs::*;
pub use project::*;
use serde::{de::DeserializeOwned, Serialize};
pub use storage::*;
pub use supavisor::*;

macro_rules! error {
    ($($arg:tt)*) => {
        crate::error::with_context(format_args!($($arg)*))
    };
}

const BASE_URL: &str = "https://api.supabase.com/v1";
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

    pub(crate) async fn send_request<T: DeserializeOwned>(
        &self,
        builder: reqwest::RequestBuilder,
    ) -> Result<T, Error> {
        let builder = builder.bearer_auth(&self.api_key);
        send_request(builder).await
    }

    pub(crate) async fn get<T: DeserializeOwned>(
        &self,
        endpoint: fmt::Arguments<'_>,
    ) -> Result<T, Error> {
        let url = format!("{BASE_URL}/{}", endpoint);
        // Use send_request so that the bearer token is applied.
        self.send_request(CLIENT.get(url)).await
    }

    pub(crate) async fn post<T: DeserializeOwned>(
        &self,
        endpoint: fmt::Arguments<'_>,
        payload: Option<impl Serialize>,
    ) -> Result<T, Error> {
        let url = format!("{BASE_URL}/{}", endpoint);
        let mut builder = CLIENT.post(url);
        if let Some(payload) = payload {
            builder = builder.json(&payload);
        }
        self.send_request(builder).await
    }

    pub(crate) async fn put<T: DeserializeOwned>(
        &self,
        endpoint: fmt::Arguments<'_>,
        payload: Option<impl Serialize>,
    ) -> Result<T, Error> {
        let url = format!("{BASE_URL}/{}", endpoint);
        let mut builder = CLIENT.put(url);
        if let Some(payload) = payload {
            builder = builder.json(&payload);
        }
        self.send_request(builder).await
    }
}

pub(crate) async fn send_request<T: DeserializeOwned>(
    builder: reqwest::RequestBuilder,
) -> Result<T, Error> {
    let resp = builder
        .send()
        .await
        .map_err(|err| error!("Failed to send request: {err}"))?;
    let status = resp.status();

    if status.is_client_error() || status.is_server_error() {
        let code = status.as_u16();
        let msg = resp
            .text()
            .await
            .map_err(|err| Error(format!("Failed to get text from response: {err}").into()))?;
        return Err(error!("{code}: {msg}"));
    }

    resp.json().await.map_err(|err| {
        error!(
            "Failed to parse JSON into {}: {err}",
            std::any::type_name::<T>()
        )
    })
}
