use std::sync::LazyLock;

use regex::Regex;
use std::str::Lines;
use tower_lsp_server::lsp_types::{Position, Range};

use crate::types::{ChangeBlock, Conflict};
use crate::utils::get_utf16_len;

static DIFF_CONFLICT_START_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^<{7,} [Cc]onflict \d+ of \d+$").unwrap());
static DIFF_CONFLICT_END_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^>{7,} [Cc]onflict \d+ of \d+ ends$").unwrap());
static DIFF_CHANGE_HEADER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^%{7,} .+$").unwrap());
static DIFF_CONTENTS_HEADER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+{7,} .+$").unwrap());
static DIFF_CONTINUATION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\\{7,}\s").unwrap());

pub struct Analyzer<'a> {
    conflicts: Vec<Conflict>,
    lines: Lines<'a>,
    cur_line: Option<&'a str>,
    cur_line_number: u32,
}

impl<'a> Analyzer<'a> {
    pub fn new(content: &'a str) -> Self {
        Analyzer {
            conflicts: vec![],
            lines: content.lines(),
            cur_line: None,
            cur_line_number: 0,
        }
    }

    pub fn find_conflicts(&'a mut self) -> &'a [Conflict] {
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
        self.skip_continuation_lines();

        let mut content = String::new();
        let mut line = self.cur_line?;

        while !is_known_pattern(line) {
            if line.starts_with("-") {
                line = self.next()?;
                continue;
            }

            if !content.is_empty() {
                content.push('\n');
            }

            if let Some(line_content) = line.strip_prefix("+") {
                content.push_str(line_content);
            } else {
                content.push_str(line);
            }

            line = self.next()?;
        }

        Some(ChangeBlock {
            title_range,
            content,
        })
    }

    fn parse_contents_block(&mut self) -> Option<ChangeBlock> {
        let title_range = self.get_range_of_current_line()?;
        self.next()?;
        self.skip_continuation_lines();

        let mut content = String::new();
        let mut line = self.cur_line?;

        while !is_known_pattern(line) {
            if !content.is_empty() {
                content.push('\n');
            }
            content.push_str(line);
            line = self.next()?;
        }

        Some(ChangeBlock {
            title_range,
            content,
        })
    }

    /// Skip continuation lines (e.g. `\\\\\\\ to: ...` in jj 0.37+)
    fn skip_continuation_lines(&mut self) {
        while let Some(line) = self.cur_line {
            if DIFF_CONTINUATION_REGEX.is_match(line) {
                self.next();
            } else {
                break;
            }
        }
    }

    fn next(&mut self) -> Option<&'a str> {
        if self.cur_line.is_some() {
            self.cur_line_number += 1;
        }
        self.cur_line = self.lines.next();
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
        || DIFF_CONTINUATION_REGEX.is_match(content)
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
        let mut analyzer = Analyzer::new(&content);
        let conflicts = analyzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_diff_three_sides() {
        let content = fs::read_to_string("tests/conflicts/diff/three_sides.md")
            .expect("Failed to read input file");
        let mut analyzer = Analyzer::new(&content);
        let conflicts = analyzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_diff_four_sides() {
        let content = fs::read_to_string("tests/conflicts/diff/four_sides.md")
            .expect("Failed to read input file");
        let mut analyzer = Analyzer::new(&content);
        let conflicts = analyzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_diff_whole_file_detailed() {
        let content = fs::read_to_string("tests/conflicts/diff/whole_file_detailed.md")
            .expect("Failed to read input file");
        let mut analyzer = Analyzer::new(&content);
        let conflicts = analyzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_diff_two_sides_detailed() {
        let content = fs::read_to_string("tests/conflicts/diff/two_sides_detailed.md")
            .expect("Failed to read input file");
        let mut analyzer = Analyzer::new(&content);
        let conflicts = analyzer.find_conflicts();
        assert_debug_snapshot!(conflicts);
    }

    #[test]
    fn test_regex_patterns() {
        let tests = [
            // Classic format (jj < 0.37)
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
            // Detailed format (jj 0.37+)
            (DIFF_CONFLICT_START_REGEX.clone(), "<<<<<<< conflict 1 of 2"),
            (
                DIFF_CONFLICT_END_REGEX.clone(),
                ">>>>>>> conflict 2 of 2 ends",
            ),
            (
                DIFF_CHANGE_HEADER_REGEX.clone(),
                r#"%%%%%%% diff from: rlvkpnrz 2f040964 (rebased revision's parent)"#,
            ),
            (
                DIFF_CONTENTS_HEADER_REGEX.clone(),
                r#"+++++++ zsuskuln f7705e4f (rebased revision)"#,
            ),
            // Longer conflict markers (jj 0.25+)
            (
                DIFF_CONFLICT_START_REGEX.clone(),
                "<<<<<<<< conflict 1 of 2",
            ),
            (
                DIFF_CONFLICT_END_REGEX.clone(),
                ">>>>>>>> conflict 1 of 2 ends",
            ),
            (
                DIFF_CHANGE_HEADER_REGEX.clone(),
                "%%%%%%%% Changes from base to side #1",
            ),
            (
                DIFF_CONTENTS_HEADER_REGEX.clone(),
                "++++++++ Contents of side #2",
            ),
        ];

        for (regex_pattern, haystack) in tests {
            assert!(
                regex_pattern.is_match(haystack),
                "expected pattern to match: {haystack}"
            );
        }
    }
}
