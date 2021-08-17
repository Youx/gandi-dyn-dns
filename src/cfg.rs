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
        log::info!(target: "cfg", "reading cfg file {}", filename);
        match std::fs::read_to_string(&filename) {
            Err(e) => {
                log::error!(target: "cfg", "failed to read file {}: {}", filename, e);
                panic!();
            }
            Ok(s) => {
                match toml::from_str::<Cfg>(&s) {
                    Err(e) => {
                        log::error!(target: "cfg", "failed to parse config: {}", e);
                        panic!();
                    }
                    Ok(cfg) => {
                        log::debug!(target: "cfg", "loaded config: {:#?}", cfg);
                        cfg
                    }
                }
            }
        }
    }
}
