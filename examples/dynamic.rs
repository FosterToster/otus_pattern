use otus_pattern::dyn_impl::traits::Component;
use otus_pattern::dyn_impl::types::{File, Folder};
fn main() {
    let folder = Folder {
        name: "root".to_string(),
        children: vec![
            Box::new(File(".gitignore".to_string())),
            Box::new(File("Cargo.toml".to_string())),
            Box::new(Folder {
                name: "src".to_string(),
                children: vec![
                    Box::new(File("lib.rs".to_string())),
                    Box::new(Folder {
                        name: "dyn_impl".to_string(),
                        children: vec![
                            Box::new(File("mod.rs".to_string())),
                            Box::new(File("traits.rs".to_string())),
                            Box::new(File("types.rs".to_string())),
                        ],
                    }),
                    Box::new(Folder {
                        name: "static_impl".to_string(),
                        children: vec![
                            Box::new(File("mod.rs".to_string())),
                            Box::new(File("traits.rs".to_string())),
                            Box::new(File("types.rs".to_string())),
                        ],
                    }),
                ],
            }),
            Box::new(Folder {
                name: "examples".to_string(),
                children: vec![
                    Box::new(File("static.rs".to_string())),
                    Box::new(File("dynamic.rs".to_string())),
                ],
            }),
        ],
    };

    println!("{}", folder.tree_view());
}
