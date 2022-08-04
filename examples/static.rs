use otus_pattern::static_impl::traits::{CompositeLeaf, CompositeItem};
use otus_pattern::static_impl::types::{Paragraph, Sentence};

fn main() {

    let article = Paragraph(vec![
        CompositeItem::Leaf(Sentence("Static implementation".to_string())),
        CompositeItem::Container(Paragraph(vec![
            CompositeItem::Leaf(Sentence("This is static implementation of Composite pattern.".to_string())),
            CompositeItem::Leaf(Sentence("In this expamle you will see how to use it.".to_string())),
            CompositeItem::Container(Paragraph(vec![
                CompositeItem::Leaf(Sentence("Trait \"CompositeLeaf\" defines a single method - \"operation\" and associative type of its return.".to_string())),
                CompositeItem::Leaf(Sentence("Enum \"CompositeItem\" describes leaf, container and operation result generic types.".to_string())),
                CompositeItem::Leaf(Sentence("Finally \"CompositeContainer\" trait defines method which is needed to be able to append concrete CompositeItem to its storage.".to_string())),
            ]))
        ])),
        CompositeItem::Container(Paragraph(vec![
            CompositeItem::Leaf(Sentence("Enjoy!".to_string()))
        ]))
    ]);

    println!("{}", article.operation())
}
