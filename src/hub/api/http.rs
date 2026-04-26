//! Minimal HTTP server adapter.
//!
//! Binds a [`TcpListener`], parses incoming HTTP/1.1 requests, and
//! delegates them to a user-supplied handler function.

use std::io::{Read, Write};
use std::net::TcpListener;

/// Minimal single-threaded HTTP/1.1 server.
pub struct HttpServer {
    addr: String,
}

impl HttpServer {
    /// Creates a server bound to the given address.
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
        }
    }

    /// Accepts connections and dispatches them to `handler(method, path, body)`.
    pub fn bind_and_serve(
        &self,
        handler: &dyn Fn(&str, &str, &str) -> HttpResponse,
    ) -> std::io::Result<()> {
        let listener = TcpListener::bind(&self.addr)?;
        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buf = [0u8; 8192];
            let n = stream.read(&mut buf)?;
            let raw = String::from_utf8_lossy(&buf[..n]);
            let (method, path, body) = parse_request(&raw);
            let response = handler(&method, &path, &body);
            let header = format!(
                "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                response.status_code,
                response.status_text,
                response.content_type,
                response.body.len()
            );
            stream.write_all(header.as_bytes())?;
            stream.write_all(response.body.as_bytes())?;
        }
        Ok(())
    }
}

/// HTTP response returned by the handler.
pub struct HttpResponse {
    /// HTTP status code.
    pub status_code: u16,
    /// HTTP status text.
    pub status_text: String,
    /// MIME content type.
    pub content_type: String,
    /// Response body.
    pub body: String,
}

impl HttpResponse {
    /// Returns a 200 OK JSON response.
    pub fn ok(body: &str) -> Self {
        Self {
            status_code: 200,
            status_text: "OK".into(),
            content_type: "application/json".into(),
            body: body.to_string(),
        }
    }

    /// Returns a 400 Bad Request JSON response.
    pub fn bad_request(body: &str) -> Self {
        Self {
            status_code: 400,
            status_text: "Bad Request".into(),
            content_type: "application/json".into(),
            body: body.to_string(),
        }
    }

    /// Returns a 404 Not Found JSON response.
    pub fn not_found() -> Self {
        Self {
            status_code: 404,
            status_text: "Not Found".into(),
            content_type: "application/json".into(),
            body: r#"{"error":"not found"}"#.to_string(),
        }
    }

    /// Returns a 500 Internal Server Error JSON response.
    pub fn internal_error(msg: &str) -> Self {
        Self {
            status_code: 500,
            status_text: "Internal Server Error".into(),
            content_type: "application/json".into(),
            body: format!(r#"{{"error":"{msg}"}}"#),
        }
    }
}

fn parse_request(raw: &str) -> (String, String, String) {
    let mut lines = raw.lines();
    let first = lines.next().unwrap_or("");
    let parts: Vec<&str> = first.split_whitespace().collect();
    let method = parts.first().unwrap_or(&"GET").to_string();
    let path = parts.get(1).unwrap_or(&"/").to_string();
    let body = if let Some(pos) = raw.find("\r\n\r\n") {
        raw[pos + 4..].to_string()
    } else {
        String::new()
    };
    (method, path, body)
}
