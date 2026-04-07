use reqwest::Url;
use reqwest::cookie::Jar;
use std::fs::read_to_string;

pub struct Cookie;
impl Cookie {
    pub fn from_file(cookie_file: &str) -> Jar {
        let jar = Jar::default();
        for line in read_to_string(cookie_file).expect("Couldnt read cookie file").lines() {
            if line.starts_with("#") || line.trim().is_empty() {
                continue;
            }

            let parts: Vec<_> = line.split("\t").collect();

            if parts.len() < 7 {
                continue;
            }

            let domain = parts[0].trim_start_matches(".");
            let name = parts[5];
            let value = parts[6];

            let protocol = if parts[3] == "TRUE" { "https" } else { "http" };
            let url = Url::parse(&format!("{}://{}", protocol, domain)).unwrap();

            let cookie_str = format!("{}={}; Domain={}; Path={}", name, value, domain, parts[2]);

            jar.add_cookie_str(&cookie_str, &url);
        }
        jar
    }
}
