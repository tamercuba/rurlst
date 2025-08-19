use std::{
    error::Error,
    fmt,
    net::{SocketAddr, ToSocketAddrs},
    str::FromStr,
    vec,
};

#[derive(Debug, Clone)]
pub enum AddressParsingError {
    EmptyHostname,
    InvalidPort(String),
    PortOutOfRange(i32),
}

impl fmt::Display for AddressParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AddressParsingError::EmptyHostname => write!(f, "Hostname não pode estar vazio"),
            AddressParsingError::InvalidPort(p) => write!(f, "Porta inválida: {}", p),
            AddressParsingError::PortOutOfRange(p) => {
                write!(f, "Porta fora do range (1-65535): {}", p)
            }
        }
    }
}

impl Error for AddressParsingError {}

#[derive(Clone, Debug)]
pub struct Address {
    hostname: String,
    port: i32,
    path: String,
}

impl Address {
    pub fn new(hostname: &str, port: Option<i32>, path: &str) -> Self {
        Address {
            hostname: hostname.into(),
            port: match port {
                Some(v) => v,
                None => 80,
            },
            path: path.into(),
        }
    }
}

impl ToSocketAddrs for &Address {
    type Iter = vec::IntoIter<SocketAddr>;

    fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
        format!("{}:{}", self.hostname, self.port).to_socket_addrs()
    }
}

impl FromStr for Address {
    type Err = AddressParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = s
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let default_port = if s.starts_with("https://") { 443 } else { 80 };

        let (host_part, path) = match url.find("/") {
            Some(idx) => {
                let h = &url[..idx];
                if h.is_empty() {
                    return Err(AddressParsingError::EmptyHostname);
                }

                (&url[..idx], url[idx..].to_string())
            }
            None => (url, String::from("/")),
        };

        let (host, port) = match host_part.rfind(":") {
            Some(idx) => {
                let hostname = &host_part[..idx];
                let port_str = &host_part[idx + 1..];

                if hostname.is_empty() {
                    return Err(AddressParsingError::EmptyHostname);
                }

                let port = port_str
                    .parse::<i32>()
                    .map_err(|_| AddressParsingError::InvalidPort(port_str.to_string()))?;

                if !(1..=65535).contains(&port) {
                    return Err(AddressParsingError::PortOutOfRange(port));
                }

                (hostname.to_string(), port)
            }
            None => (host_part.to_string(), default_port),
        };

        Ok(Address::new(&host, Some(port), &path))
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Address: {}", self.hostname)?;
        writeln!(f, "Port: {}", self.port)?;
        writeln!(f, "Path: {}", self.path)?;
        Ok(())
    }
}
