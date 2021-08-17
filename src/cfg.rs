// gandi-dyn-dns: dynamically refresh DNS records for gandi.net
// Copyright (C) 2021 Hugo Camboulive <hugo@cambou.live>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

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
