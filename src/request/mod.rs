use native_tls::TlsConnector;
use std::io::{Read, Write};

fn unchunk(raw: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let mut cursor = 0;
    let mut out = Vec::new();
    loop {
        let line_end = raw[cursor..]
            .windows(2)
            .position(|w| w == b"\r\n")
            .ok_or("unexpected EOF")?;

        let size = std::str::from_utf8(&raw[cursor..cursor + line_end])?;
        let chunk_size = usize::from_str_radix(size.trim(), 16)?;

        cursor += line_end + 2;
        if chunk_size == 0 {
            break;
        }
        out.extend_from_slice(&raw[cursor..cursor + chunk_size]);
        cursor += chunk_size + 2;
    }
    Ok(String::from_utf8(out)?)
}

pub fn req(query: &str, lang: &str) -> Result<String, Box<dyn std::error::Error>> {
    let connector = TlsConnector::new()?;
    let host = format!("{}.wiktionary.org", lang);
    let stream = std::net::TcpStream::connect(format!("{}:443", host))?;
    let mut stream = connector.connect(&host, stream)?;

    let encoded: String = query
        .chars()
        .flat_map(|c| {
            if c.is_ascii_alphanumeric() || "-_.~".contains(c) {
                vec![c]
            } else {
                c.to_string()
                    .bytes()
                    .flat_map(|b| {
                        const HEX: &[u8] = b"0123456789abcdef";
                        vec![
                            '%',
                            HEX[(b >> 4) as usize] as char,
                            HEX[(b & 0xf) as usize] as char,
                        ]
                    })
                    .collect()
            }
        })
        .collect();

    let path = format!(
        "/w/api.php?action=query&format=json&prop=revisions&titles={}&formatversion=2&rvprop=content&rvslots=main",
        encoded
    );

    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: slown/0.1\r\nConnection: close\r\n\r\n",
        path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut raw = Vec::new();
    stream.read_to_end(&mut raw)?;

    let body_start = raw
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|i| i + 4)
        .ok_or("no header separator found")?;
    let is_chunked = raw
        .windows(b"Transfer-Encoding: chunked".len())
        .any(|w| w.eq_ignore_ascii_case(b"Transfer-Encoding: chunked"));

    println!("chunked: {}", is_chunked);

    if is_chunked {
        unchunk(&raw[body_start..])
    } else {
        Ok(String::from_utf8(raw[body_start..].to_vec())?)
    }
}
