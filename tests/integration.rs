use supabase_management_rs::Client;

#[tokio::test]
async fn integration_tests() -> Result<(), supabase_management_rs::Error> {
    let client_id = std::env::var("SUPABASE_CLIENT_ID").unwrap();
    let client_secret = std::env::var("SUPABASE_CLIENT_SECRET").unwrap();
    let refresh_token = std::env::var("SUPABASE_REFRESH_TOKEN").unwrap();

    let token_resp =
        supabase_management_rs::generate_access_token(&client_id, &client_secret, &refresh_token)
            .await?;

    println!("New refresh token: {:?}", token_resp.refresh_token);

    let client = Client::new(token_resp.access_token);

    let projects = client.list_projects().await?;

    for project in projects {
        println!("Project: {project:?}");

        dbg!(client.get_postgres_config(&project.id).await?);
        dbg!(client.get_supavisor_details(&project.id).await?);
        dbg!(client.get_project_health(&project.id).await?);
        dbg!(client.get_storage_config(&project.id).await?);
    }

    Ok(())
}
