extern crate chrono;
extern crate rss;

use chrono::DateTime;
use rss::Channel;
use std::cmp::Ordering;
use std::fs;

struct FeedItem {
    title: String,
    link: String,
    pub_date: String,
}

pub fn create_readme() -> std::io::Result<()> {
    let tpl =
        fs::read_to_string("README.md.tpl").expect("Something went wrong reading the README.tpl file");
    let last_articles = get_latest_articles();

    return fs::write(
        "README.md",
        tpl.replace("%{{latest_articles}}%", &last_articles),
    );
}

fn get_latest_articles() -> String {
    let mut posts: Vec<FeedItem> = get_blog_rss();

    posts.sort_by(|a, b| {
        let date_a = DateTime::parse_from_rfc2822(&a.pub_date).unwrap();
        let date_b = DateTime::parse_from_rfc2822(&b.pub_date).unwrap();

        if date_b < date_a {
            Ordering::Less
        } else if date_b > date_a {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });

    return posts[..5].iter().fold("".to_string(), |acc, item| {
        format!("{} \n* [{}]({})", acc, item.title, item.link)
    });
}

fn get_blog_rss() -> Vec<FeedItem> {
    let items = Channel::from_url("https://aralroca.com/rss.xml")
        .unwrap()
        .items()
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            pub_date: item.pub_date().unwrap().to_string(),
        })
        .collect();

    items
}
