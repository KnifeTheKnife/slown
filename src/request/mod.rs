use native_tls::TlsConnector;
use std::io::{Read, Write};

pub fn req(query: &str, lang: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let connector = TlsConnector::new()?;
    let stream = std::net::TcpStream::connect("208.80.154.224:443")?;
    let host = format!("{}.wiktionary.org", lang);
    let mut stream = connector.connect(&host, stream)?;

    let path = format!(
        "/w/api.php?action=query&format=json&errorformat=wikitext&prop=revisions&titles={}&formatversion=2&rvprop=content&rvslots=main",
        query
    );

    let request = format!(
        "GET {} HTTP/1.1\r\n\
        Host: {}\r\n\
        User-Agent: rust-wiktionary-client\r\n\
        Connection: close\r\n\
        \r\n",
        path, host
    );

    stream.write_all(request.as_bytes())?;

    let mut response = vec![];
    stream.read_to_end(&mut response)?;

    Ok(response)
}
// pub fn req(query: &str) -> Result<String, Box<dyn std::error::Error>> {
//     let path = format!(
//         "https://en.wiktionary.org/w/api.php?action=query&format=json&errorformat=wikitext&prop=revisions&titles={}&formatversion=2&rvprop=content&rvslots=main",
//         query
//     );
//     let response = ureq::get(&path)
//         .set("User-Agent", "rust-wiktionary-client/1.0")
//         .call()?;
//     Ok(response.into_string()?)
// }
