use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub port: u16,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("config.yml", config::FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}
