use super::traits::Component;
use std::boxed::Box;
use std::vec::Vec;

pub struct File(pub String);

impl Component<String> for File {
    fn tree_view(&self) -> String {
        self.0.clone()
    }
}

pub struct Folder {
    pub name: String,
    pub children: Vec<Box<dyn Component<String>>>,
}

impl Component<String> for Folder {
    fn tree_view(&self) -> String {
        format!(
            " ■ {}{}",
            self.name,
            self.children
                .iter()
                .map(|child| {
                    child
                        .tree_view()
                        .lines()
                        .map(|s| format!(" │ {}", s))
                        .fold(String::new(), |a, b| format!("{}\n{}", a, b))
                })
                .fold(String::new(), |r, cur| { format!("{}{}", r, cur) })
        )
    }
}
