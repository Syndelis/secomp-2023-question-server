use std::net::SocketAddr;

#[derive(clap::Parser, Debug)]
pub struct Config {
    /// The connection URL for the Postgres database this application should use.
    #[clap(long, env)]
    pub database_url: String,

    #[clap(long, env)]
    pub bind_address: SocketAddr,

}

#[cfg(test)]
mod tests {

    use clap::Parser;

    use super::*;
    use std::env;

    #[test]
    fn test_loading_config_from_environment() {

        env::set_var("DATABASE_URL", "postgres://localhost:5432");
        env::set_var("BIND_ADDRESS", "0.0.0.0:8080");

        let config = Config::parse();

        assert_eq!(config.database_url, "postgres://localhost:5432");

    }

}