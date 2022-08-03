use super::traits::{ArticleItem, ItemContainer};
use std::vec::Vec;

pub struct Sentence(pub String);

impl ArticleItem for Sentence {
    fn get_text(&self) -> String {
        self.0.clone()
    }
}

pub struct Paragraph<T>(pub Vec<T>);

impl<T: ArticleItem> ArticleItem for Paragraph<T> {
    fn get_text(&self) -> String {
        self.0.iter().fold(String::new(), |r, member| {
            format!("{}\n{}", r, member.get_text())
        })
    }
}

impl<T: ArticleItem> ItemContainer<T> for Paragraph<T> {
    fn append_item(&mut self, item: T) {
        self.0.push(item)
    }
}

pub struct Article<T> {
    header: String,
    members: Vec<T>,
}

impl<T> Article<T> {
    pub fn new(header: String, members: Vec<T>) -> Self {
        Self { header, members }
    }
}

impl<T: ArticleItem> ArticleItem for Article<T> {
    fn get_text(&self) -> String {
        format!(
            "{}\n{}",
            self.header,
            self.members.iter().fold(String::new(), |r, member| format!(
                "{}\n{}",
                r,
                member.get_text()
            ))
        )
    }
}
