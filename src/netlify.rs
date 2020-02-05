use failure::{bail, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DNSRecord {
    pub hostname: String,
    #[serde(rename = "type")]
    pub dns_type: String,
    pub ttl: Option<u32>,
    pub id: Option<String>,
    pub value: String,
}

/// Retrieve the DNS records for domain, authenticated with token.
pub fn get_dns_records(domain: &str, token: &str) -> Result<Vec<DNSRecord>, Error> {
    #[cfg(not(test))]
    let url = format!(
        "https://api.netlify.com/api/v1/dns_zones/{}/dns_records",
        domain.replace(".", "_"),
    );
    #[cfg(test)]
    let url = {
        let _ = domain; // supress unused variable warning in test
        mockito::server_url()
    };

    let resp = ureq::get(&url).query("access_token", token).call();

    if resp.ok() {
        let dns_records: Vec<DNSRecord> = serde_json::from_str(&resp.into_string()?)?;
        Ok(dns_records)
    } else {
        bail!(
            "[{}] ({}): Unable to get DNS records.",
            resp.status(),
            resp.status_text()
        );
    }
}

/// Delete the DNS record.
pub fn delete_dns_record(domain: &str, token: &str, record: DNSRecord) -> Result<(), Error> {
    #[cfg(not(test))]
    let url = format!(
        "https://api.netlify.com/api/v1/dns_zones/{}/dns_records/{}",
        domain.replace(".", "_"),
        record.id.expect("Record did not have an ID."),
    );
    #[cfg(test)]
    let url = {
        let _ = (domain, record); // supress unused variable warning in test
        mockito::server_url()
    };

    let response = ureq::delete(&url).query("access_token", token).call();
    match response.status() {
        404 => bail!("The domain {} could not be found on your account.", domain),
        401 => bail!("Unauthorized credentials. Check your Netlify token"),
        200 | 204 => (),
        status => bail!("Error {}: could not delete the dns record", status),
    }

    Ok(())
}

/// Add a dns record to the domain.
pub fn add_dns_record(domain: &str, token: &str, record: DNSRecord) -> Result<DNSRecord, Error> {
    #[cfg(not(test))]
    let url = format!(
        "https://api.netlify.com/api/v1/dns_zones/{}/dns_records",
        domain.replace(".", "_"),
    );
    #[cfg(test)]
    let url = {
        let _ = domain; // supress unused variable warning in test
        mockito::server_url()
    };

    let mut req = ureq::post(&url)
        .query("access_token", token)
        .set("Content-Type", "application/json")
        .build();
    let resp = req.send_string(&serde_json::to_string(&record)?);

    let result: DNSRecord = match resp.status() {
        201 => serde_json::from_str(&resp.into_string()?)?,
        404 => bail!("The domain {} could not be found on your account.", domain),
        401 => bail!("Unauthorized credentials. Check your Netlify token"),
        status => bail!("Error {}: could not add the dns record", status),
    };

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use mockito::{mock, Matcher};

    #[test]
    fn test_get_dns_records() {
        let body = "[{\"hostname\":\"www.example.com\",\"type\":\"NETLIFY\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"\",\"site_id\":\"\",\"dns_zone_id\":\"\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"},{\"hostname\":\"example.com\",\"type\":\"NETLIFY\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"\",\"site_id\":\"\",\"dns_zone_id\":\"\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"},{\"hostname\":\"www.example.com\",\"type\":\"NETLIFYv6\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"\",\"site_id\":\"\",\"dns_zone_id\":\"\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"},{\"hostname\":\"example.com\",\"type\":\"NETLIFYv6\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"\",\"site_id\":\"\",\"dns_zone_id\":\"\",\"errors\":[],\"managed\":true,\"value\":\"example.netlify.com\"}]";

        let _m = mock("GET", "/")
            .match_query(Matcher::Regex("access_token.+$".into()))
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_header("content-length", &body.len().to_string())
            .with_body(&body)
            .create();

        let dns_records = get_dns_records("example.com", "token").unwrap();
        assert_eq!(dns_records.len(), 4);
    }

    #[test]
    fn test_delete_dns_records() {
        let _m = mock("DELETE", "/")
            .match_query(Matcher::Regex("access_token.+$".into()))
            .with_status(200)
            .create();

        assert!(delete_dns_record(
            "example.com",
            "token",
            DNSRecord {
                hostname: String::from(""),
                dns_type: String::from(""),
                ttl: None,
                id: Some(String::from("example")),
                value: String::from("example"),
            }
        )
        .is_ok());

        let _m = mock("DELETE", "/")
            .match_query(Matcher::Regex("access_token.+$".into()))
            .with_status(404)
            .create();

        assert!(delete_dns_record(
            "example.com",
            "token",
            DNSRecord {
                hostname: String::from(""),
                dns_type: String::from(""),
                ttl: None,
                id: Some(String::from("example")),
                value: String::from("example"),
            }
        )
        .is_err());
    }

    #[test]
    fn test_add_dns_records() {
        let body = "{\"hostname\":\"test.example.com\",\"type\":\"A\",\"ttl\":3600,\"priority\":null,\"weight\":null,\"port\":null,\"flag\":null,\"tag\":null,\"id\":\"\",\"site_id\":null,\"dns_zone_id\":\"\",\"errors\":[],\"managed\":false,\"value\":\"192.0.0.1\"}";

        let _m = mock("POST", "/")
            .match_query(Matcher::Regex("access_token.+$".into()))
            .match_header("Content-Type", "application/json")
            .with_status(201)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_header("content-length", &body.len().to_string())
            .with_body(&body)
            .create();

        let resp = add_dns_record(
            "example.com",
            "token",
            DNSRecord {
                hostname: String::from(""),
                dns_type: String::from(""),
                ttl: None,
                id: Some(String::from("")),
                value: String::from(""),
            },
        )
        .unwrap();
        assert_eq!(resp.hostname, String::from("test.example.com"));
        assert_eq!(resp.dns_type, String::from("A"));
        assert_eq!(resp.ttl, Some(3600));
        assert_eq!(resp.value, String::from("192.0.0.1"));
        assert_eq!(resp.id, Some(String::from("")));
    }
}
