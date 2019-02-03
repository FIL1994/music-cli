use dialoguer::{theme::ColorfulTheme, Select};
use std::string::{String};
use walkdir::WalkDir;

fn main() {
    let files = WalkDir::new("dir")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| match e.path().extension() {
            Some(ext_osstr) => match ext_osstr.to_str() {
                Some(ext) => {
                    let extensions: [String; 1] = ["abc".to_string()];
                    if extensions.contains(&ext.to_ascii_lowercase()) {
                        Some(e)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        });

    for entry in files {
//        println!("{}", entry.path().display());

        let i = entry.path().file_name().unwrap();
        println!("file name: {}",
                 i.to_str().unwrap().replace(".", "")
        );

//        println!("{:?}", entry);
    }

    let selections = &[
        "Ice Cream",
        "Vanilla Cupcake",
        "Chocolate Muffin",
        "A Pile of sweet, sweet mustard",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your flavor")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();
    println!("Enjoy your {}!", selections[selection]);
}
