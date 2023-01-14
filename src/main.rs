use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use tokio;

error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

// async main function tokio
#[tokio::main]
async fn main() -> Result<()> {
    let body = reqwest::get("https://www.rust-lang.org/")
        .await?
        .text()
        .await?;

    let document = Document::from(&body[..]);

    // get all hrefs
    for node in document.find(Name("a")) {
        println!("{}", node.attr("href").unwrap());
    }

    Ok(())
}
