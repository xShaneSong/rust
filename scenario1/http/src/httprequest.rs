use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Resuource {
    Path(String),
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub resource: Resuource,
    pub version: Version,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::V1_1;
        let mut parsed_resource = Resuource::Path(String::new());
        let mut parsed_headers = HashMap::new();
        let mut parsed_msg_body = String::new(); // 修改为 String::new()

        for line in req.lines() {
            if line.is_empty() {
                continue; // Skip empty lines
            }
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line); // 修复变量名：medthod -> method
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else {
                parsed_msg_body = line.to_string(); // 修改为 line.to_string()
            }
        }
        
        // 将 HttpRequest 创建移到循环外部
        HttpRequest {
            method: parsed_method,
            resource: parsed_resource,
            version: parsed_version,
            headers: parsed_headers,
            msg_body: parsed_msg_body,
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resuource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let resource = words.next().unwrap();
    let version = words.next().unwrap();

    (
        method.into(),
        Resuource::Path(resource.to_string()),
        version.into(),
    )
}

fn process_header_line(s: &str) -> (String, String) {
    let mut header_items = s.split(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some(k) = header_items.next() {
        key = k.trim().to_string();
    }
    if let Some(v) = header_items.next() {
        value = v.trim().to_string();
    }
    (key, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from_str() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
        assert_eq!(Method::from("GET"), Method::Get);
        assert_eq!(Method::from("POST"), Method::Post);
        assert_eq!(Method::from("PUT"), Method::Uninitialized);
        assert_eq!(Method::from("DELETE"), Method::Uninitialized);
    }

    #[test]
    fn test_version_from_str() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("HTTP/3.0"), Version::Uninitialized);
        assert_eq!(Version::from("HTTP/0.9"), Version::Uninitialized);
    }

    #[test]
    fn test_http_request_from_string() {
        let req_str = "GET /index.html HTTP/1.1\r\nHost: example.com\r\nUser-Agent: test\r\n\r\n";
        let req: HttpRequest = req_str.to_string().into();
        
        assert_eq!(req.method, Method::Get);
        assert_eq!(req.resource, Resuource::Path("/index.html".to_string()));
        assert_eq!(req.version, Version::V1_1);
        assert_eq!(req.headers.get("Host").unwrap(), "example.com");
        assert_eq!(req.headers.get("User-Agent").unwrap(), "test");
        assert_eq!(req.msg_body, "");
    }
}