use unicode_width::UnicodeWidthStr;

use std::fmt::Write;

#[derive(Debug)]
pub struct Lines {
    lines: Vec<Line>,
}

#[derive(Debug)]
struct Group {
    pub depth: usize,
    pub start: usize,
    pub end: usize,
    pub width: usize,
}

impl Lines {
    pub fn new(source: String) -> Lines {
        let lines: Vec<Line> = source
            .lines()
            .map(|line| Line::new(line.to_string()))
            .collect();
        let mut tabstops_lines = Lines { lines: lines };

        let mut groups = Vec::new();
        for i in 0..tabstops_lines.max_depth() {
            groups.append(&mut tabstops_lines.groups(i));
        }
        tabstops_lines.update_width(groups);
        tabstops_lines
    }

    fn max_depth(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.blocks.len())
            .max()
            .unwrap()
    }

    fn groups(&self, depth: usize) -> Vec<Group> {
        let mut group_tuples = Vec::new();

        let mut start: Option<usize> = Option::None;
        let mut end: Option<usize> = Option::None;
        let mut current_max_width: usize = 0;

        for (i, line) in self.lines.iter().enumerate() {
            let tab_break_line = match line.blocks.get(depth) {
                Option::None => true,
                Option::Some(block) => {
                    if block.has_next && current_max_width < block.width {
                        current_max_width = block.width;
                    }
                    let is_empty_block = block.block_string == "";
                    is_empty_block
                }
            };
            if tab_break_line {
                if let Some(group) = self.new_group(start, end, depth, current_max_width) {
                    group_tuples.push(group)
                }
                start = Option::None;
                current_max_width = 0;
            }
            if start.is_none() {
                start = Option::Some(i);
            }
            end = Option::Some(i);
        }
        if let Some(group) = self.new_group(start, end, depth, current_max_width) {
            group_tuples.push(group)
        }

        group_tuples
    }

    fn new_group(
        &self,
        start: Option<usize>,
        end: Option<usize>,
        depth: usize,
        width: usize,
    ) -> Option<Group> {
        start
            .map(|start| {
                end.map(|end| Group {
                    depth: depth,
                    start: start,
                    end: end,
                    width: width,
                })
            })
            .flatten()
    }

    fn update_width(&mut self, groups: Vec<Group>) {
        for group in groups {
            for line_index in group.start..=group.end {
                self.lines
                    .get_mut(line_index)
                    .unwrap()
                    .set_adjust_width(group.depth, group.width)
            }
        }
    }

    pub fn to_string(self) -> String {
        let mut result = String::new();
        for line in self.lines {
            for block in line.blocks {
                write!(
                    result,
                    "{space:<indent$}",
                    space = block.block_string,
                    indent = block.width_with_margin(1, 4)
                )
                .unwrap();
            }
            writeln!(result).unwrap();
        }
        result
    }
}

#[derive(Debug)]
struct Line {
    blocks: Vec<Block>,
}

impl Line {
    fn new(line: String) -> Line {
        let block_strs: Vec<String> = line.split("\t").map(|block| block.to_string()).collect();
        let block_strs_max_index = block_strs.len() - 1;
        let mut blocks = Vec::new();
        for i in 0..block_strs.len() {
            let block_str = block_strs.get(i).unwrap();
            let has_next = i != block_strs_max_index;
            blocks.push(Block {
                adjust_width: 0,
                has_next: has_next,
                width: block_str.width_cjk(),
                block_string: block_str.to_string(),
            })
        }
        Line { blocks: blocks }
    }

    fn set_adjust_width(&mut self, block_index: usize, adjust_width: usize) {
        self.blocks
            .get_mut(block_index)
            .map(|block| block.adjust_width = adjust_width);
    }
}

#[derive(Debug)]
struct Block {
    pub adjust_width: usize,
    pub width: usize,
    pub has_next: bool,
    pub block_string: String,
}

impl Block {
    pub fn width_with_margin(&self, margin: usize, empty_width: usize) -> usize {
        if self.has_next {
            if self.adjust_width == 0 {
                empty_width
            } else {
                self.adjust_width + margin
            }
        } else {
            self.width
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tabstops::Lines;

    #[test]
    fn test_simple() {
        assert(
            "\
var hoge\t= 123;
var mogegegegege\t= 234;
var a\t= 345;
",
            "\
var hoge         = 123;
var mogegegegege = 234;
var a            = 345;
",
        );
    }

    #[test]
    fn test_tsv() {
        assert(
            "\
positive\tinterest\tleaving\tbat\tgolden\tfeel
news\tfinest\tearth\tbut\tpeace\twall
hard\tmountain\tcheese\tpupil\trailroad\twhistle
largest\tlength\trefer\talso\tletter\ttaken
easily\tjet\tyoung\talready\tsoap\tgulf
fast\tdirt\tbasis\thow\tlibrary\tflame
",
            "\
positive interest leaving bat     golden   feel
news     finest   earth   but     peace    wall
hard     mountain cheese  pupil   railroad whistle
largest  length   refer   also    letter   taken
easily   jet      young   already soap     gulf
fast     dirt     basis   how     library  flame
",
        );
    }

    #[test]
    fn test_source() {
        assert(
            "\
function hoge() {
\tvar x = 0;\t/* comment1 */
\tvar xxxyyyzzz = 2;\t/* comment2 */
}
",
            "\
function hoge() {
    var x = 0;         /* comment1 */
    var xxxyyyzzz = 2; /* comment2 */
}
",
        );
    }

    fn assert(input: &str, expect: &str) {
        assert_eq!(
            Lines::new(String::from(input)).to_string(),
            String::from(expect)
        );
    }
}
