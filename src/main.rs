extern crate rss;

use rss::Channel;
use std::fs;

struct FeedItem {
    title: String,
    link: String,
    desc: String,
    pub_date: String,
}

fn print_item(item: &FeedItem) {
    println!("{}", item.title);
    println!("{}", item.link);
    println!("");
}

fn get_blog_rss() -> Vec<FeedItem> {
    let items = Channel::from_url("https://aralroca.com/rss.xml")
        .unwrap()
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            desc: item.description().unwrap().to_string(),
            pub_date: item.pub_date().unwrap().to_string(),
        })
        .collect();

    items
}

fn get_latest_articles() -> String {
    let posts: Vec<FeedItem> = get_blog_rss();

    return posts.iter().fold("".to_string(), |acc, item| {
        format!("{} \n* [{}]({})", acc, item.title, item.link)
    });
}

fn create_readme() {
    let tpl =
        fs::read_to_string("README.tpl").expect("Something went wrong reading the README.tpl file");
    let last_articles = get_latest_articles();

    fs::write(
        "README.md",
        tpl.replace("%{{latest_articles}}%", &last_articles),
    );
}

fn main() {
    create_readme();
}
