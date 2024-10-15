//! src/configuration.rs
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connect_string(&self) -> String {
        //postgresql://[User]:[Password]@[Host]:[Port]/[Database]
        /*https://xata.io/docs/postgres*/
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

pub fn configuration_get() -> Result<Settings, config::ConfigError> {
    //Initialise our configuration reader
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name(
        "/home/tuetd/Desktop/rust_playground/Rustingg/actix-web/configuration.yaml",
    ))?;

    settings.try_into()
}
