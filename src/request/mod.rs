use std::io::{Read, Result, Write};
// Establish connection with wikitionary. Takes url as input, returns stream struct
fn est() -> Result<std::net::TcpStream> {
    std::net::TcpStream::connect("208.80.154.224:80")
}
// Perform the API call to search for the query, returns string
pub fn req(query: &str) -> Result<String> {
    let mut stream = est()?;
    let path = format!(
        "/w/api.php?action=query&format=json&errorformat=wikitext&prop=revisions&titles={}&formatversion=2&rvprop=content&rvslots=main",
        query
    );

    let request = format!(
        "GET {} HTTP/1.1\r\n\
        Host: en.wiktionary.org\r\n\
        User-Agent: rust-wiktionary-client\r\n\
        Connection: close\r\n\
        \r\n",
        path
    );
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();

    stream.read_to_string(&mut response)?;

    if let Some(body_start) = response.find("\r\n\r\n") {
        Ok(response[body_start + 4..].to_string())
    } else {
        Ok(response)
    }
}
