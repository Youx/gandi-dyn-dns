use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PrefixCfg {
    pub name: String,
    pub ipv4: bool,
    pub ipv6: bool,
}

#[derive(Deserialize, Debug)]
pub struct DomainCfg {
    pub name: String,
    pub prefixes: Vec<PrefixCfg>,
}

#[derive(Deserialize, Debug)]
pub struct Cfg {
    pub api_key: String,
    pub domains: Vec<DomainCfg>,
    pub refresh_interval: u16,
}

impl Cfg {
    pub fn load(filename: String) -> Self {
        let s = std::fs::read_to_string(filename).unwrap();
        toml::from_str(&s).unwrap()
    }
}
