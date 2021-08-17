use serde::Serialize;

#[derive(Debug)]
pub enum DnsRecordType {
    A,
    AAAA,
}

pub struct Client {
    base_url: &'static str,
    api_key: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct LiveDnsPost {
    rrset_values: Vec<String>,
}

impl Client {
    async fn put(&self, url: String, json: &impl Serialize)
        -> Result<reqwest::Response, reqwest::Error>
    {
        self.client.put(format!("{}{}", &self.base_url, &url))
            .header(reqwest::header::AUTHORIZATION, &format!("ApiKey {}", &self.api_key))
            .json(json)
            .send()
            .await
    }

    async fn post(&self, url: String, json: &impl Serialize)
        -> Result<reqwest::Response, reqwest::Error>
    {
        self.client.post(format!("{}{}", &self.base_url, url))
            .header(reqwest::header::AUTHORIZATION, &format!("ApiKey {}", &self.api_key))
            .json(json)
            .send()
            .await
    }

    pub async fn update_dns_entry(&self, domain: &str, name: &str, kind: DnsRecordType, value: &str)
        -> Result<(), reqwest::Error>
    {
        let url = format!("/livedns/domains/{}/records/{}/{:?}", domain, name, kind);
        let request = LiveDnsPost { rrset_values: vec![String::from(value)] };
        let data = self.put(url, &request).await?;

        match data.status() {
            reqwest::StatusCode::CREATED => {
                log::info!(target: "livedns",
                           "updated DNS entry {:?} {}.{} to {}",
                           kind, name, domain, value);
                Ok(())
            },
            e => {
                log::error!(target: "livedns",
                            "unhandled error {}: {}", e.as_u16(), data.text().await?);
                panic!();
            },
        }
    }

    pub async fn create_dns_entry(&self, domain: &str, name: &str, kind: DnsRecordType, value: &str)
        -> Result<(), reqwest::Error>
    {
        let url = format!("/livedns/domains/{}/records/{}/{:?}", domain, name, kind);
        let request = LiveDnsPost { rrset_values: vec![String::from(value)] };
        let data = self.post(url, &request).await?;

        match data.status() {
            reqwest::StatusCode::OK => {
                log::info!(target: "livedns", "unchanged DNS entry {:?} {}.{} ({})",
                           kind, name, domain, value);
                Ok(())
            },
            reqwest::StatusCode::CREATED => {
                log::info!(target: "livedns", "created DNS entry {:?} {}.{} to {}", kind, name, domain, value);
                Ok(())
            },
            reqwest::StatusCode::CONFLICT => self.update_dns_entry(domain, name, kind, value).await,
            e => {
                log::error!(target: "livedns", "unhandled error {}: {}",
                            e.as_u16(), data.text().await?);
                panic!();
            }
        }
    }

    pub fn new(api_key: String) -> Self {
        Client {
            base_url: "https://api.gandi.net/v5",
            client: reqwest::Client::new(),
            api_key: api_key,
        }
    }
}
