use serde::{Serialize, Deserialize};

pub struct Client {
    client: reqwest::Client,
}

#[derive(Serialize,Deserialize)]
pub struct Ips {
    pub v4: Option<String>,
    pub v6: Option<String>,
}

impl Ips {
    pub fn new() -> Self {
        return Ips {
            v4: None,
            v6: None,
        }
    }
}

#[derive(Deserialize)]
struct IpifyResult {
    ip: String,
}

impl Client {
    pub fn new() -> Self {
        Client {
            client: reqwest::Client::new()
        }
    }

    async fn get_ipv4(&self) -> Result<String, reqwest::Error> {
        let res: IpifyResult = self.client.get("https://api.ipify.org/?format=json")
            .send().await?
            .json().await?;
        log::debug!(target: "ipify", "fetched IPv4: {}", res.ip);
        Ok(res.ip)
    }

    async fn get_ipv6(&self) -> Result<String, reqwest::Error> {
        let res: IpifyResult = self.client.get("https://api6.ipify.org/?format=json")
            .send().await?
            .json().await?;
        log::debug!(target: "ipify", "fetched IPv6: {}", res.ip);
        Ok(res.ip)
    }

    pub async fn get(&self, ipv4: bool, ipv6: bool, res: &mut Ips) -> Result<(), reqwest::Error> {
        if ipv4 && res.v4.is_none() {
            res.v4 = Some(self.get_ipv4().await?);
        }
        if ipv6 && res.v6.is_none() {
            res.v6 = Some(self.get_ipv6().await?);
        }
        Ok(())
    }
}
