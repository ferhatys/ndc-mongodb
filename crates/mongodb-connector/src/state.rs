use std::{env, error::Error};

use anyhow::anyhow;
use mongodb_agent_common::{interface_types::MongoConfig, mongodb_connection::get_mongodb_client};

pub const DATABASE_URI_ENV_VAR: &str = "MONGODB_DATABASE_URI";

/// Reads database connection URI from environment variable
pub async fn try_init_state() -> Result<MongoConfig, Box<dyn Error + Send + Sync>> {
    // Splitting this out of the `Connector` impl makes error translation easier
    let database_uri = env::var(DATABASE_URI_ENV_VAR)?;
    let client = get_mongodb_client(&database_uri).await?;
    let database_name = match client.default_database() {
        Some(database) => Ok(database.name().to_owned()),
        None => Err(anyhow!(
            "${DATABASE_URI_ENV_VAR} environment variable must include a database"
        )),
    }?;
    Ok(MongoConfig {
        client,
        database: database_name,
    })
}
