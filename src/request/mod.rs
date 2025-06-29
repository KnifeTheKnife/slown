use ureq;
// NO UREQ IMPLEMENTATION (MAYBE WILL GET IT WORKING LATER)
// Establish connection with wiktionary. Takes url as input, returns stream struct
// fn est() -> Result<std::net::TcpStream, Box<dyn std::error::Error>> {
//     Ok(std::net::TcpStream::connect("208.80.154.224:80")?)
// }
// Perform the API call to search for the query, returns string !!NO TLS!!
// pub fn req(query: &str) -> Result<String> {
//     let mut stream = est()?;
//     let path = format!(
//         "/w/api.php?action=query&format=json&errorformat=wikitext&prop=revisions&titles={}&formatversion=2&rvprop=content&rvslots=main",
//         query
//     );

//     let request = format!(
//         "GET {} HTTP/1.1\r\n\
//         Host: en.wiktionary.org\r\n\
//         User-Agent: rust-wiktionary-client\r\n\
//         Connection: close\r\n\
//         \r\n",
//         path
//     );
//     stream.write_all(request.as_bytes())?;

//     let mut response = String::new();

//     stream.read_to_string(&mut response)?;

//     Ok(response)
// }
pub fn req(query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let path = format!(
        "https://en.wiktionary.org/w/api.php?action=query&format=json&errorformat=wikitext&prop=revisions&titles={}&formatversion=2&rvprop=content&rvslots=main",
        query
    );
    let response = ureq::get(&path)
        .set("User-Agent", "rust-wiktionary-client/1.0")
        .call()?;
    Ok(response.into_string()?)
}
