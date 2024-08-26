use anyhow::anyhow;

#[derive(Debug, PartialEq)]
pub struct Packet {
    version: u8,
    method: Method,
    error: Error,
    body: String,
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Method {
    Query,
    Answer,
}

impl TryFrom<u8> for Method {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Method::Query),
            1 => Ok(Method::Answer),
            _ => Err(anyhow!("Invalid method")),
        }
    }
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Error {
    None,
    Parse,
    NotFound,
    WrongVersion,
    InternalServer,
}

impl TryFrom<u8> for Error {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Error::None),
            1 => Ok(Error::Parse),
            2 => Ok(Error::NotFound),
            3 => Ok(Error::WrongVersion),
            4 => Ok(Error::InternalServer),
            _ => Err(anyhow!("Invalid error")),
        }
    }
}

impl TryFrom<&[u8]> for Packet {
    type Error = anyhow::Error;

    fn try_from(buf: &[u8]) -> std::result::Result<Self, Self::Error> {
        if buf.len() < 3 {
            return Err(anyhow!("Packet muse be at least 4 bytes long"));
        }

        let version = buf[0];
        let method = buf[1];
        let error = buf[2];
        let body = String::from_utf8(buf[3..].to_vec())?;

        Ok(Packet {
            version,
            method: Method::try_from(method)?,
            error: Error::try_from(error)?,
            body,
        })
    }
}

impl From<Packet> for Vec<u8> {
    fn from(val: Packet) -> Self {
        let mut buf = Vec::with_capacity(3 + val.body.len());
        buf.push(val.version);
        buf.push(val.method as u8);
        buf.push(val.error as u8);
        buf.extend_from_slice(val.body.as_bytes());

        buf
    }
}

impl Packet {
    pub fn new(version: u8, method: Method, error: Error, body: String) -> Self {
        Self {
            version,
            method,
            error,
            body,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet_parser() {
        let buf = [1u8, 0, 0, 72, 105];
        let packet = Packet::try_from(&buf as &[u8]);
        let expected = Packet {
            version: 1,
            method: Method::Query,
            error: Error::None,
            body: "Hi".to_string(),
        };

        assert!(packet.is_ok());
        assert_eq!(expected, packet.unwrap());
    }

    #[test]
    fn test_struct_to_vec() {
        let packet = Packet {
            version: 1,
            method: Method::Query,
            error: Error::None,
            body: "Hi".to_string(),
        };
        let expected = vec![1u8, 0, 0, 72, 105];
        let buf: Vec<u8> = packet.into();

        assert_eq!(expected, buf);
    }
}
