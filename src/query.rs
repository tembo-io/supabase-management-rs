use serde::{de::DeserializeOwned, Serialize};

use crate::Client;

impl Client {
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
    ) -> Result<T, crate::Error> {
        #[derive(Serialize)]
        struct Body<'a> {
            query: &'a str,
        }

        // CLIENT
        //     .post(&url)
        //     .bearer_auth(&self.api_key)
        //     .json(&Body { query })
        //     .send()
        //     .await?
        //     .json()
        //     .await

        self.post(
            format_args!("projects/{project_id}/database/query"),
            Some(&Body { query }),
        )
        .await
    }
}
