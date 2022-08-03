use otus_pattern::static_impl::traits::ArticleItem;
use otus_pattern::static_impl::types::{Article, Paragraph, Sentence};

fn main() {
    let article = Article::new(
        String::from("Static implementation"),
        vec![
            Paragraph(
                vec![
                    Sentence("This is static implementation of Composite pattern.".to_string()),
                    Sentence("In this expamle you will see how to use it.".to_string()),
                ]
            ),
            Paragraph(
                vec![
                    Sentence("Component trait \"ArticleItem\" requires a single function - get_text() -> String.".to_string()),
                    Sentence("\"ItemContainer\" trait requires \"append_item\" function to be implemented.".to_string()),
                    Sentence("\"ArticleItemContainer\" trait combines both traits above.".to_string()),
                ]
            ),
            Paragraph(
                vec![
                    Sentence("Enjoy!".to_string()),
                ]
            ),
        ]
    );

    println!("{}", article.get_text())
}
