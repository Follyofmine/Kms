// sources/mangabuddy.rs
// Aidoku source implementation for mangabuddy.com

use aidoku::prelude::*;

use aidoku::std::register_source;
use aidoku::std::net::Request;
use aidoku::std::prelude::*;
use aidoku::std::defaults::defaults;
use aidoku::std::html::Node;
use aidoku::std::error::AidokuError;
use aidoku::std::error::AidokuErrorKind;
use aidoku::std::ObjectRef;

fn search_manga(query: &str) -> Result<Vec<ObjectRef>, AidokuError> {
    let url = format!("https://mangabuddy.com/search?q={}", query);
    let html = Request::new(&url, HttpMethod::Get).string()?;
    let document = Node::new(&html);
    let mut results = Vec::new();
    for item in document.select(".book-item") {
        let title = item.select(".title h3 a").text().read();
        let url = item.select(".title h3 a").attr("href").read();
        let cover = item.select(".thumb img").attr("src").read();
        let obj = ObjectRef::new();
        obj.set("title", title);
        obj.set("url", format!("https://mangabuddy.com{}", url));
        obj.set("cover", cover);
        results.push(obj);
    }
    Ok(results)
}

pub fn register() {
    register_source!(
        "mangabuddy",
        "MangaBuddy",
        "en",
        "https://mangabuddy.com",
        |
            input, _ctx| {
                let query = input.get("query").as_string().unwrap_or_default();
                if query.is_empty() {
                    return Ok(Vec::new());
                }
                search_manga(&query)
            },
        |
            _input, _ctx| {
                // Placeholder: Return an empty manga object
                Ok(None)
            },
        |
            _input, _ctx| {
                // Placeholder: Return an empty chapter list
                Ok(Vec::new())
            },
        |
            _input, _ctx| {
                // Placeholder: Return an empty page list
                Ok(Vec::new())
            },
        defaults!()
    );
}
