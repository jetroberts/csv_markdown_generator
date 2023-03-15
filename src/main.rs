use std::{fs::File, io::BufReader, env};

mod markdown_table;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[1];
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut md_tbl = markdown_table::new_markdown_table(reader);
    md_tbl.create_markdown_table();
}
