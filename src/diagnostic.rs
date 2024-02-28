use std::process::exit;

use crate::lexer::Span;

pub struct Diagnostic<'a> {
    split_source: Vec<&'a str>,
}

impl<'a> Diagnostic<'a> {
    pub fn from_source(source: &'a str) -> Self {
        Self {
            split_source: source.split_inclusive("\r\n").collect(),
        }
    }

    fn get_line(&self, lo: usize) -> usize {
        let mut counter = 0;

        for (i, line) in self.split_source.iter().enumerate() {
            if (counter..counter + line.len()).contains(&lo) {
                return i + 1;
            }

            counter += line.len();
        }

        0
    }

    pub fn warn(&self, span: Span, message: String) {
        let line_number = self.get_line(span.lo);
        println!("warn at line {}: {}", line_number, message);
    }

    pub fn error(&self, span: Span, message: String) {
        let line_number = self.get_line(span.lo);
        println!("error at line {}: {}", line_number, message);
        exit(0);
    }
}
