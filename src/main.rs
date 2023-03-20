use std::{env, fs::File, io::BufReader};

mod markdown_table;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No file provided");
        return;
    }

    let filepath = &args[1];
    if *filepath == String::new() {
        return;
    }

    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut md_tbl = markdown_table::new_markdown_table(reader);
    md_tbl.create_markdown_table();
}
