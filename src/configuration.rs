#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: Port,
}

#[derive(serde::Deserialize)]
pub struct Port(pub u16);
#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: DatabaseUsername,
    pub password: DatabasePassword,
    pub port: DatabasePort,
    pub host: Host,
    pub database_name: DatabaseName,
}
impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username.0, self.password.0, self.host.0, self.port.0, self.database_name.0
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username.0, self.password.0, self.host.0, self.port.0
        )
    }
}
#[derive(serde::Deserialize)]
pub struct DatabaseUsername(String);
#[derive(serde::Deserialize)]
pub struct DatabasePassword(String);
#[derive(serde::Deserialize)]
pub struct DatabasePort(u16);
#[derive(serde::Deserialize)]
pub struct Host(String);
#[derive(serde::Deserialize)]
pub struct DatabaseName(pub String);

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}
