pub trait ArticleItem {
    fn get_text(&self) -> String;
}

pub trait ItemContainer<T> {
    fn append_item(&mut self, item: T);
}

pub trait ArticleItemContainer<T: ArticleItem>: ArticleItem + ItemContainer<T> {}
