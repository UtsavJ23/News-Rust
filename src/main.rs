use std::error::Error;
use serde::Deserialize;
use colour::{cyan, yellow};

#[derive(Deserialize, Debug)]
struct Articles{
    articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
struct Article{
    title: String,
    url: String
}
fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>>{
    let response = ureq::get(url).call()?.into_string()?;

    let articles:Articles = serde_json::from_str(&response)?;

    Ok(articles)
}
fn render_articles(articles: &Articles) {
    for art in &articles.articles {
        yellow!("-> {}\n", art.title);
        cyan!(": {}\n\n", art.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url: &str = "https://newsapi.org/v2/top-headlines?sources=bbc-news&apiKey=f75447654b16434e91337c2578db90d6";
    let articles : Articles = get_articles(url)?;

    render_articles(&articles);

    Ok(())

}