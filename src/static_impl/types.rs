use super::traits::{CompositeContainer, CompositeItem, CompositeLeaf};
use std::vec::Vec;

pub struct Sentence(pub String);

impl CompositeLeaf for Sentence {
    type Return = String;

    fn operation(&self) -> Self::Return {
        self.0.clone()
    }
}

pub struct Paragraph(pub Vec<CompositeItem<Sentence, Self, String>>);

impl CompositeLeaf for Paragraph {
    type Return = String;

    fn operation(&self) -> Self::Return {
        self.0.iter().fold(String::new(), |r, cur| {
            format!(
                "{}{}",
                r,
                match cur {
                    CompositeItem::Leaf(sentence) => format!("{}\n", sentence.operation()),
                    CompositeItem::Container(container) => format!("\n{}", container.operation()),
                }
            )
        })
    }
}

impl CompositeContainer<Sentence, Paragraph, String> for Paragraph {
    fn append_item(&mut self, member: CompositeItem<Sentence, Paragraph, String>) {
        self.0.push(member)
    }
}
