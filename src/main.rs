#[macro_use]
extern crate include_dir;

use include_dir::Dir;

// Include the 'templates' directory using the include_dir! macro
static TEMPLATES_DIR: Dir = include_dir!("src/templates");

fn main() {
    // Access the templates as a static Dir
    let template1 = TEMPLATES_DIR.get_file("template1.hbs").unwrap();
    let template2 = TEMPLATES_DIR.get_file("template2.hbs").unwrap();

    // Do something with the templates
    println!("Template 1:\n{}", template1.contents_utf8().unwrap());
    println!("Template 2:\n{}", template2.contents_utf8().unwrap());
}
