mod help;
mod parser;
mod request;
// TODO: Add pretty print for en
// TODO: Add pretty print for every lang
pub fn get_lang(args: &[String]) -> (&str, Option<&str>) {
    let lang = match args.get(1).map(String::as_str) {
        Some("en") => "en",
        Some("de") => "de",
        Some("ru") => "ru",
        Some("pl") => "pl",
        Some("lt") => "lt",
        Some("zh") => "zh",
        Some("es") => "es",
        Some("eo") => "eo",
        Some("fr") => "fr",
        Some("nl") => "nl",
        Some("pt") => "pt",
        Some("vo") => "vo",
        Some("sr") => "sr",
        Some("el") => "el",
        Some("ca") => "ca",
        Some("fy") => "fy",
        Some("gl") => "gl",
        Some("it") => "it",
        Some("ku") => "ku",
        Some("cs") => "cs",
        Some("la") => "la",
        Some("sk") => "sk",
        Some("tr") => "tr",
        Some("wa") => "wa",
        Some("af") => "af",
        Some("id") => "id",
        Some("ms") => "ms",
        Some("tl") => "tl",
        Some("vi") => "vi",
        Some("hu") => "hu",
        Some(_) | None => return ("en", Some("Uknown lang; No lang??")),
    };
    (lang, None)
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        help::print_help();
        return Err("Err".into());
    }
    if args[1] == "-h" || args[1] == "--help" {
        help::print_help();
        return Err("Err".into());
    }
    let (lang, err) = get_lang(&args);
    let mut arg = args[1].clone();
    if let None = err {
        arg = args[2].clone();
    } else {
        eprintln!("[ERROR]:No lang or uknown");
    }
    let body = request::req(&arg, lang)?;
    let response: parser::Response = serde_json::from_str(&body)?;

    let content = response
        .query
        .pages
        .first()
        .and_then(|p| p.revisions.as_ref())
        .and_then(|r| r.first())
        .map(|r| &r.slots.main.content)
        .ok_or("no content found")?;

    println!("{}", content);
    Ok(())
}
