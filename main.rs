use std::error::Error;
use reqwest;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let body = reqwest::blocking::get("https://feeds.npr.org/1001/rss.xml")?
        .text()?;
    let xml = body;
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);
    let mut txt = Vec::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error in reading xml {:?}", e),
            Ok(Event::Eof) => break,
            Ok(Event::Text(e)) => txt.push(e.unescape().unwrap().into_owned()),
            _ => (),
        }       
}
        buf.clear();
        for x in txt.into_iter() {
            println!("\n{:?}",x);
        }
        Ok(())
}
