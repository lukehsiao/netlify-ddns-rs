extern crate reqwest;

use failure::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DNSRecord {
    hostname: String,
    #[serde(rename = "type")]
    dns_type: String,
    ttl: u32,
    priority: Option<String>,
    weight: Option<String>,
    port: Option<String>,
    flag: Option<String>,
    tag: Option<String>,
    id: String,
    site_id: String,
    dns_zone_id: String,
    value: String,
}

/// Retrieve the DNS records for domain, authenticated with token.
pub fn get_dns_records(domain: &str, token: &str) -> Result<Vec<DNSRecord>, Error> {
    let url = format!(
        "https://api.netlify.com/api/v1/dns_zones/{}/dns_records?access_token={}",
        domain.replace(".", "_"),
        token
    );
    #[cfg(test)]
    let mut res = reqwest::get(&mockito::server_url())?;

    #[cfg(not(test))]
    let mut res = reqwest::get(url.as_str())?;

    let dns_records: Vec<DNSRecord> = res.json()?;

    Ok(dns_records)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::mock;

    #[test]
    fn test_get_dns_records() {
        let body = "[{\"hostname\":\"www.example.com\",\"type\":\"NETLIFY\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"5c3c343c50ab38c5d4b73003\",\"site_id\":\"d6e3d4f7-c8a5-44f3-90ab-2a4aa63ff52b\",\"dns_zone_id\":\"5c3c343b50ab38c5d4b73001\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"},{\"hostname\":\"example.com\",\"type\":\"NETLIFY\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"5c3c343d50ab38c5d4b73005\",\"site_id\":\"d6e3d4f7-c8a5-44f3-90ab-2a4aa63ff52b\",\"dns_zone_id\":\"5c3c343b50ab38c5d4b73001\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"},{\"hostname\":\"www.example.com\",\"type\":\"NETLIFYv6\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"5c3c3e41ccd232f0f0298fb9\",\"site_id\":\"d6e3d4f7-c8a5-44f3-90ab-2a4aa63ff52b\",\"dns_zone_id\":\"5c3c343b50ab38c5d4b73001\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"},{\"hostname\":\"example.com\",\"type\":\"NETLIFYv6\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"5c3c3e42ccd232f0f0298fbb\",\"site_id\":\"d6e3d4f7-c8a5-44f3-90ab-2a4aa63ff52b\",\"dns_zone_id\":\"5c3c343b50ab38c5d4b73001\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"}]";

        let _m = mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_header("content-length", &body.len().to_string())
            .with_body(&body)
            .create();

        let dns_records = get_dns_records("example.com", "token").unwrap();
        assert_eq!(dns_records.len(), 4);
    }
}
