use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
}

#[derive(Debug, Clone, Copy)]
pub enum MethodErrKind {
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseMethodErr {
    kind: MethodErrKind,
}

#[derive(Debug, PartialEq)]
pub struct Uri {
    pub path: String,
    pub query_params: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy)]
pub enum UriErrKind {
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseUriErr {
    kind: UriErrKind,
}

#[derive(Debug, PartialEq)]
pub struct Request {
    pub method: Method,
    pub uri: Uri,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, Clone, Copy)]
pub enum RequestErrKind {
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub struct ParseRequestErr{
    kind: RequestErrKind,
}

#[derive(Debug, PartialEq)]
pub struct Response {
    pub code: u32,
    pub message: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Display for Method {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::Get => "GET",
                Method::Head => "HEAD",
                Method::Post => "POST",
                Method::Put => "PUT",
                Method::Delete => "DELETE",
                Method::Connect => "CONNECT",
                Method::Options => "OPTIONS",
                Method::Trace => "TRACE",
                Method::Patch => "PATCH",
            }
        )
    }
}

impl Display for Uri {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.path)?;
        let mut iter = self.query_params.iter();
        if let Some((key, val)) = iter.next() {
            write!(f, "?{}={}", key, val)?;
            for (key, val) in iter {
                write!(f, "&{}={}", key, val)?;
            }
        }
        Ok(())
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} HTTP/1.1\r\n",
            self.method.to_string(),
            self.uri.to_string()
        )?;
        let mut iter = self.headers.iter();
        if let Some((key, val)) = iter.next() {
            write!(f, "{}: {}", key, val)?;
            for (key, val) in iter {
                write!(f, "\n{}: {}", key, val)?;
            }
        }
        write!(f, "\r\n{}", self.body)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "HTTP/1.1 {} {}\r\n", self.code, self.message)?;
        let mut iter = self.headers.iter();
        if let Some((key, val)) = iter.next() {
            write!(f, "{}: {}", key, val)?;
            for (key, val) in iter {
                write!(f, "\n{}: {}", key, val)?;
            }
        }
        write!(f, "\r\n{}", self.body)
    }
}

impl FromStr for Method {
    type Err = ParseMethodErr;

    fn from_str(s: &str) -> Result<Method, ParseMethodErr> {
        match s {
            "GET" => Ok(Method::Get),
            "HEAD" => Ok(Method::Head),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "CONNECT" => Ok(Method::Connect),
            "OPTIONS" => Ok(Method::Options),
            "TRACE" => Ok(Method::Trace),
            "PATCH" => Ok(Method::Patch),
            _ => Err(ParseMethodErr {
                kind: MethodErrKind::Empty,
            })
        }
    }
}

impl FromStr for Uri {
    type Err = ParseUriErr;

    fn from_str(s: &str) -> Result<Uri, ParseUriErr> {
        let err = ParseUriErr {
            kind: UriErrKind::Empty,
        };
        let param_index = s.find("?").unwrap_or(s.len());
        let path = String::from(&s[..param_index]);
        let mut query_params = HashMap::new();

        if param_index < s.len() {
            for param_pair in s[param_index+1..].split('&') {
                let mut iter = param_pair.split('=');
                let key = iter.next().ok_or(err)?;
                let value = iter.next().ok_or(err)?;

                if let Some(_) = iter.next() {
                    return Err(err);
                } else {
                    query_params.insert(String::from(key), String::from(value));
                }
            }
        }

        Ok(Uri{
            path,
            query_params,
        })
    }
}

impl FromStr for Request {
    type Err = ParseRequestErr;

    fn from_str(s: &str) -> std::result::Result<Request, ParseRequestErr> {
        let err = ParseRequestErr {
            kind: RequestErrKind::Empty,
        };
        let method_index = s.find(" ").ok_or(err)?;
        if method_index + 1 > s.len() {
            return Err(err);
        }

        let method = s[..method_index].parse().or(Err(err))?;

        let uri_index = method_index + 1 + s[method_index+1..].find(" ").ok_or(err)?;
        if uri_index + 1 > s.len() {
            return Err(err);
        }

        let uri = s[method_index+1..uri_index].parse().or(Err(err))?;
        if !s[uri_index+1..].starts_with("HTTP/") {
            return Err(err);
        }

        let header_start_index = s.find("\r\n").ok_or(err)?;
        if header_start_index + 2 > s.len() {
            return Err(err);
        }

        let body_start_index = header_start_index + 2 + s[header_start_index+2..].find("\r\n").ok_or(err)?;
        if body_start_index + 2 > s.len() {
            return Err(err);
        }

        let mut headers = HashMap::new();

        let header_text = &s[header_start_index+2..body_start_index];
        for header_pair in header_text.split('\n') {
            let mut iter = header_pair.split(": ");
            let key = iter.next().ok_or(err)?;
            let value = iter.next().ok_or(err)?;
            if let Some(_) = iter.next() {
                return Err(err);
            }
            headers.insert(String::from(key), String::from(value));
        }

        let body = if body_start_index + 2 >= s.len() {
            String::from("")
        } else {
            String::from(&s[body_start_index+2..])
        };

        Ok(Request {
            method,
            uri,
            headers,
            body,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn request_formatted_properly() {
        let request = Request {
            method: Method::Get,
            uri: Uri {
                path: String::from("/"),
                query_params: {
                    let mut map = HashMap::new();
                    map.insert(String::from("user"), String::from("test"));
                    map
                },
            },
            headers: {
                let mut map = HashMap::new();
                map.insert(String::from("User-Agent"), String::from("Rust"));
                map
            },
            body: String::from(""),
        };

        assert_eq!(
            "GET /?user=test HTTP/1.1\r\nUser-Agent: Rust\r\n",
            request.to_string()
        );
    }

    #[test]
    fn response_formatted_properly() {
        let response = Response {
            code: 200,
            message: String::from("OK"),
            headers: {
                let mut map = HashMap::new();
                map.insert(String::from("Host"), String::from("192.168.1.123"));
                map
            },
            body: String::from(""),
        };
        assert_eq!(
            "HTTP/1.1 200 OK\r\nHost: 192.168.1.123\r\n",
            response.to_string()
        );
    }

    #[test]
    fn request_parsed_properly() {
        let request: Request = "GET /index.html?user=test HTML/1.1\r\nUser-Agent: Rust\r\nHello, world!"
            .parse()
            .unwrap();
        assert_eq!(Request {
            method: Method::Get,
            uri: Uri {
                path: String::from("/index.html"),
                query_params: {
                    let mut map = HashMap::new();
                    map.insert(String::from("user"), String::from("test"));
                    map
                },
            },
            headers: {
                let mut map = HashMap::new();
                map.insert(String::from("User-Agent"), String::from("Rust"));
                map
            },
            body: String::from("Hello, world!"),
        }, request);
    }
}
