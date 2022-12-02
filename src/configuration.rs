use config;
use serde_aux::field_attributes::deserialize_number_from_string;
use std::str::FromStr;
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSetting,
}

#[derive(serde::Deserialize)]
pub struct ApplicationSetting {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: Port,
    pub host: ApplicationHost,
}
#[derive(serde::Deserialize)]
pub struct ApplicationHost(pub String);

#[derive(serde::Deserialize)]
pub struct Port(pub u16);
impl FromStr for Port {
    type Err = <u16 as ::core::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = u16::from_str(s)?;
        Ok(Port(n))
    }
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: DatabaseUsername,
    pub password: DatabasePassword,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: DatabasePort,
    pub host: ApplicationHost,
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
impl FromStr for DatabasePort {
    type Err = <u16 as ::core::str::FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = u16::from_str(s)?;
        Ok(DatabasePort(n))
    }
}
#[derive(serde::Deserialize)]
pub struct Host(String);
#[derive(serde::Deserialize)]
pub struct DatabaseName(pub String);

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();
    let base_path = std::env::current_dir().expect("Failed to determine the curernt directory");
    let configuration_directory = base_path.join("configuration");

    settings.merge(config::File::from(configuration_directory.join("base")).required(true))?;

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");
    settings.merge(
        config::File::from(configuration_directory.join(environment.as_str())).required(true),
    )?;

    settings.merge(config::Environment::with_prefix("app").separator("__"))?;

    settings.try_into()
}
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either 'local' or 'production'",
                other
            )),
        }
    }
}
