# supabase-management-rs

A Rust client library for the [Supabase Management API](https://supabase.com/docs/reference/api/introduction).

⚠️ **Note: This crate is still a work in progress and not all API endpoints are implemented yet.**


## Usage

```rust
use supabase_management_rs::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with your Supabase management API key
    let client = Client::new("your-api-key".to_string());

    // List all projects
    let projects = client.list_projects().await?;

    // Get the first project
    if let Some(project) = projects.first() {
        println!("Project name: {}", project.name);

        // Check project health
        let health = client.get_project_health(&project.id).await?;
        println!("Project health: {:?}", health);

        // Execute a query
        let results: serde_json::Value = client
            .query(&project.id, "SELECT now()")
            .await?;
        println!("Query result: {:?}", results);

        // Pause a project
        client.pause_project(&project.id).await?;
    }

    Ok(())
}
```

### Executing Queries

You can execute PostgreSQL queries on your project and deserialize the results into your own types:

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Row {
    id: i32,
    hash_value: String,
}

let rows: Vec<Row> = client
    .query(
        project_id,
        "SELECT generate_series(1, 3) AS id, \
        md5(generate_series(1, 3)::text) AS hash_value",
    )
    .await?;

println!("{:?}", rows);
```

## API Key

To obtain an API key for the Supabase Management API, refer to the [official documentation](https://supabase.com/docs/reference/api/introduction).
