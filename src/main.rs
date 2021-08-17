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

mod livedns;
mod ipify;
mod cfg;

use clap::{Arg, App};
extern crate pretty_env_logger;

async fn refresh_ips(
    cfg: &cfg::Cfg,
    dns_client: &livedns::Client,
    ip_client: &ipify::Client)
    -> Result<(), reqwest::Error>
{
    let mut ips = ipify::Ips::new();

    for domain in &cfg.domains {
        for prefix in &domain.prefixes {
            ip_client.get(prefix.ipv4, prefix.ipv6, &mut ips).await?;

            if prefix.ipv4 {
                if let Some(ref ipv4) = ips.v4 {
                    dns_client.create_dns_entry(
                        &domain.name,
                        &prefix.name,
                        livedns::DnsRecordType::A,
                        &ipv4
                    ).await?;
                }
            }

            if prefix.ipv6 {
                if let Some(ref ipv6) = ips.v6 {
                    dns_client.create_dns_entry(
                        &domain.name,
                        &prefix.name,
                        livedns::DnsRecordType::AAAA,
                        &ipv6
                    ).await?;
                }
            }
        }
    };

    Ok(())
}

fn get_default_config_file() -> std::path::PathBuf {
    match dirs::config_dir() {
        None => std::path::PathBuf::from(r"gandi-dyn-dns.toml"),
        Some(mut path) => {
            path.push("gandi-dyn-dns");
            path.push("gandi-dyn-dns.toml");
            path
        }
    }
}

#[tokio::main]
pub async fn main() {
    let matches = App::new("LiveDNS")
        .version("0.1.0")
        .author("Hugo Camboulive <hugo@cambou.live>")
        .about("Periodically update IP on Gandi using LiveDNS API")
        .arg(Arg::with_name("config")
                 .short("c")
                 .long("config")
                 .takes_value(true)
                 .help("A configuration file"))
        .after_help(
"LOGGING:
    Launch with environment variable RUST_LOG=<level> or RUST_LOG=<logger>=<level>
    Available levels: error, warning, info, debug (default: error)
    Available loggers: cfg, ipify, livedns
"
        )
        .get_matches();

    pretty_env_logger::init();

    let default_conf = get_default_config_file();
    let conf_file = matches.value_of("config")
        .unwrap_or(default_conf.to_str().unwrap());
    let cfg = cfg::Cfg::load(String::from(conf_file));
    
    let livedns = livedns::Client::new(String::clone(&cfg.api_key));
    let ipify = ipify::Client::new();
    
    log::info!("IPs will be refreshed every {} minutes",
               cfg.refresh_interval);

    let mut interval = tokio::time::interval(
        std::time::Duration::from_secs(
            cfg.refresh_interval as u64 * 60
        )
    );

    loop {
        interval.tick().await;
        match refresh_ips(&cfg, &livedns, &ipify).await {
            Err(e) => {
                log::error!("error refreshing dns records: {}", e);
            }
            Ok(_) => {}
        }
    }
}
