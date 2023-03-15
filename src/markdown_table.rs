use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Seek},
};

const BAR: &str = "|";
const SEP: &str = "\t";

pub struct MarkdownTable {
    reader: BufReader<File>,
}

pub fn new_markdown_table(reader: BufReader<File>) -> MarkdownTable {
    return MarkdownTable { reader };
}

impl MarkdownTable {
    pub fn create_markdown_table(&mut self) -> Vec<u8> {
        let mut count = 0;
        let mut output_line: String = String::new();

        let col_count = get_col_count(self.reader.by_ref());

        let mut max_padding: Vec<usize> = vec![0; col_count];
        for line in self.reader.by_ref().lines() {
            let current_line = line.unwrap();
            let words = current_line.split(SEP);

            for (index, word) in words.enumerate() {
                let word_len = word.len();
                if word_len > max_padding[index] {
                    max_padding[index] = word_len;
                }
            }
        }

        self.reader.rewind().unwrap();

        for line in self.reader.by_ref().lines() {
            let cur_line = line.unwrap();
            if count < 1 {
                let col_count = cur_line.split(SEP).count();
                let header = create_line(&cur_line, &max_padding).to_owned();
                let header_line = create_header_line(col_count, &max_padding);

                output_line.push_str(&header);
                output_line.push_str(&header_line);
                count += 1;
                continue;
            }

            let new_line = create_line(&cur_line, &max_padding).to_owned();
            output_line.push_str(&new_line);
            count += 1;
        }

        println!("{}", output_line);

        return vec![0x0a];
    }
}

fn get_col_count(line_iter: &mut BufReader<File>) -> usize {
    let first_line = line_iter.lines().next();
    line_iter.rewind().unwrap();
    let header = match first_line {
        Some(h) => h.unwrap(),
        None => "".to_string(),
    };

    return header.split(SEP).count();
}

fn create_line(line: &str, padding: &Vec<usize>) -> String {
    let words = line.split(SEP);
    let mut padded_words = vec![String::new(); padding.len()];

    for (index, word) in words.enumerate() {
        padded_words[index] = format!("{:width$}", word, width = padding[index]);
    }

    return format!("|{}|\n", padded_words.join(BAR));
}

fn create_header_line(col_count: usize, padding: &Vec<usize>) -> String {
    let mut cols = String::new();
    for index in 0..col_count {
        // need to create a bar that has two colons at either end, padded by
        let test = format!(":{:-<width$}:|", "", width = padding[index] - 2);
        cols.push_str(&test)
    }
    return format!("|{}\n", cols);
}
