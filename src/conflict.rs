use crate::types::{ChangeBlock, Conflict};
use regex::Regex;
use std::str::Lines;
use tower_lsp_server::lsp_types::{Position, Range};

use crate::utils::get_utf16_len;

lazy_static::lazy_static! {
    static ref DIFF_CONFLICT_START_REGEX: Regex =
        Regex::new(r"^<<<<<<< Conflict \d+ of \d+$").unwrap();
    static ref DIFF_CONFLICT_END_REGEX: Regex =
        Regex::new(r"^>>>>>>> Conflict \d+ of \d+ ends$").unwrap();
    static ref DIFF_CHANGE_HEADER_REGEX: Regex =
        Regex::new(r"^%%%%%%% Changes from (base(?: #\d+)? to )?side #\d+$").unwrap();
    static ref DIFF_CONTENTS_HEADER_REGEX: Regex =
        Regex::new(r"^\+{7} Contents of side #\d+$").unwrap();
}

pub struct Analyzer<'a> {
    conflicts: Vec<Conflict>,
    lines: Lines<'a>,
    cur_line: Option<&'a str>,
    next_line: Option<&'a str>,
    cur_line_number: u32,
}

impl<'a> Analyzer<'a> {
    pub fn new(content: &'a str) -> Self {
        let mut lines = content.lines();
        let cur_line = lines.next();
        let next_line = lines.next();

        Analyzer {
            conflicts: vec![],
            lines,
            cur_line,
            next_line,
            cur_line_number: 0,
        }
    }

    pub fn find_conflicts(&'a mut self) -> &'a Vec<Conflict> {
        while let Some(line) = self.next() {
            if DIFF_CONFLICT_START_REGEX.is_match(line) {
                self.parse_diff_marker();
            }
        }
        &self.conflicts
    }

    fn parse_diff_marker(&mut self) {
        let title_range = self.get_range_of_current_line().unwrap();
        self.next();
        let mut blocks = vec![];

        while let Some(cur_line) = self.cur_line {
            if DIFF_CHANGE_HEADER_REGEX.is_match(cur_line) {
                match self.parse_change_block() {
                    Some(block) => blocks.push(block),
                    None => return,
                }
            } else if DIFF_CONTENTS_HEADER_REGEX.is_match(cur_line) {
                match self.parse_contents_block() {
                    Some(block) => {
                        blocks.push(block);
                    }
                    None => return,
                }
            } else {
                break;
            }
        }

        if let Some(cur_line) = self.cur_line {
            let end_position = Position::new(self.cur_line_number, get_utf16_len(cur_line));

            let conflict = Conflict {
                range: Range::new(title_range.start, end_position),
                title_range,
                blocks,
            };
            self.conflicts.push(conflict);
        }
    }

    fn parse_change_block(&mut self) -> Option<ChangeBlock> {
        let title_range = self.get_range_of_current_line()?;
        self.next();

        let mut content = String::new();
        let mut next_line = self.cur_line?;

        while !is_known_pattern(next_line) {
            if let Some(line_content) = next_line.strip_prefix("+") {
                if !content.is_empty() {
                    content.push('\n');
                }
                content.push_str(line_content);
            }
            next_line = self.next()?;
        }

        let block = ChangeBlock {
            title_range,
            content,
        };

        Some(block)
    }

    fn parse_contents_block(&mut self) -> Option<ChangeBlock> {
        let title_range = self.get_range_of_current_line()?;

        self.next()?;

        let mut content = String::new();
        let mut next_line = self.cur_line?;

        while !is_known_pattern(next_line) {
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(next_line);
            next_line = self.next()?;
        }

        let block = ChangeBlock {
            title_range,
            content,
        };

        Some(block)
    }

    fn next(&mut self) -> Option<&'a str> {
        self.cur_line = self.next_line;
        self.next_line = self.lines.next();

        if self.cur_line.is_some() {
            self.cur_line_number += 1;
        }

        self.cur_line
    }

    fn get_range_of_current_line(&self) -> Option<Range> {
        Some(Range {
            start: Position::new(self.cur_line_number, 0),
            end: Position::new(self.cur_line_number, get_utf16_len(self.cur_line?)),
        })
    }
}

fn is_known_pattern(content: &str) -> bool {
    DIFF_CHANGE_HEADER_REGEX.is_match(content)
        || DIFF_CONTENTS_HEADER_REGEX.is_match(content)
        || DIFF_CONFLICT_END_REGEX.is_match(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use std::fs;

    #[test]
    fn test_diff_two_sides() {
        let content = fs::read_to_string("tests/conflicts/diff/two_sides.md")
            .expect("Failed to read input file");
        let mut anayzer = Analyzer::new(&content);
        let conflicts = anayzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_diff_three_sides() {
        let content = fs::read_to_string("tests/conflicts/diff/three_sides.md")
            .expect("Failed to read input file");
        let mut anayzer = Analyzer::new(&content);
        let conflicts = anayzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_diff_four_sides() {
        let content = fs::read_to_string("tests/conflicts/diff/four_sides.md")
            .expect("Failed to read input file");
        let mut anayzer = Analyzer::new(&content);
        let conflicts = anayzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_regex_patterns() {
        let tests = [
            (DIFF_CONFLICT_START_REGEX.clone(), "<<<<<<< Conflict 1 of 2"),
            (
                DIFF_CONFLICT_END_REGEX.clone(),
                ">>>>>>> Conflict 2 of 2 ends",
            ),
            (
                DIFF_CHANGE_HEADER_REGEX.clone(),
                "%%%%%%% Changes from base to side #1",
            ),
            (
                DIFF_CHANGE_HEADER_REGEX.clone(),
                "%%%%%%% Changes from base #1 to side #1",
            ),
            (
                DIFF_CONTENTS_HEADER_REGEX.clone(),
                "+++++++ Contents of side #2",
            ),
        ];

        for (regex_pattern, haystack) in tests {
            assert!(regex_pattern.is_match(haystack))
        }
    }
}
