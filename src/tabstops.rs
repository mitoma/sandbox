use unicode_width::UnicodeWidthStr;

use std::fmt::{Display, Formatter, Result};

pub trait BlockCalcurator {
    fn calc_block_width(&self, block_string: String) -> usize;
    fn split_line(&self, line: String) -> Vec<String>;
    fn margin(&self) -> usize;
    fn tabsize(&self) -> usize;
}

struct DefaultBlockCalcurator {
    margin: usize,
    tabsize: usize,
}

impl BlockCalcurator for DefaultBlockCalcurator {
    fn calc_block_width(&self, block_string: String) -> usize {
        block_string.width_cjk()
    }

    fn split_line(&self, line: String) -> Vec<String> {
        line.split('\t').map(|block| block.to_string()).collect()
    }

    fn margin(&self) -> usize {
        self.margin
    }

    fn tabsize(&self) -> usize {
        self.tabsize
    }
}

pub struct Lines {
    lines: Vec<Line>,
    block_calcurator: Box<dyn BlockCalcurator>,
}

impl Display for Lines {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for line in self.lines.as_slice() {
            for block in line.blocks.as_slice() {
                write!(
                    f,
                    "{space:<indent$}",
                    space = block.block_string,
                    indent = block.width_with_margin(
                        self.block_calcurator.margin(),
                        self.block_calcurator.tabsize()
                    )
                )
                .unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl Lines {
    pub fn new(source: String) -> Lines {
        Self::new_with_calcurator(
            source,
            Box::new(DefaultBlockCalcurator {
                margin: 1,
                tabsize: 4,
            }),
        )
    }

    pub fn new_with_calcurator(source: String, calcurator: Box<dyn BlockCalcurator>) -> Lines {
        let vec_line: Vec<Line> = source
            .lines()
            .map(|line| Line::new(line.to_string(), calcurator.as_ref()))
            .collect();
        let mut lines = Lines {
            lines: vec_line,
            block_calcurator: calcurator,
        };

        let groups = Group::new_groups(&lines);
        lines.update_width(groups);
        lines
    }

    fn max_depth(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.blocks.len())
            .max()
            .unwrap()
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
}

struct Group {
    pub depth: usize,
    pub start: usize,
    pub end: usize,
    pub width: usize,
}

impl Group {
    fn new_groups(lines: &Lines) -> Vec<Group> {
        let mut groups = Vec::new();
        for i in 0..lines.max_depth() {
            groups.append(&mut Self::groups(lines, i));
        }
        groups
    }

    fn groups(lines: &Lines, depth: usize) -> Vec<Group> {
        let mut group_tuples = Vec::new();

        let mut start: Option<usize> = None;
        let mut end: Option<usize> = None;
        let mut current_max_width: usize = 0;

        for (i, line) in lines.lines.iter().enumerate() {
            let tab_break_line = match line.blocks.get(depth) {
                None => true,
                Some(block) => {
                    if block.has_next && current_max_width < block.width {
                        current_max_width = block.width;
                    }
                    block.block_string.is_empty()
                }
            };
            if tab_break_line {
                if let (Some(start), Some(end)) = (start, end) {
                    group_tuples.push(Self::new_group(start, end, depth, current_max_width));
                }
                start = None;
                current_max_width = 0;
            }
            if start.is_none() {
                start = Some(i);
            }
            end = Some(i);
        }

        if let (Some(start), Some(end)) = (start, end) {
            group_tuples.push(Self::new_group(start, end, depth, current_max_width));
        }

        group_tuples
    }

    fn new_group(start: usize, end: usize, depth: usize, width: usize) -> Group {
        Group {
            depth,
            start,
            end,
            width,
        }
    }
}

struct Line {
    blocks: Vec<Block>,
}

impl Line {
    fn new(line: String, calcurator: &dyn BlockCalcurator) -> Line {
        let block_strs: Vec<String> = calcurator.split_line(line);
        let block_strs_max_index = block_strs.len() - 1;
        let mut blocks = Vec::new();
        for i in 0..block_strs.len() {
            let block_str = block_strs.get(i).unwrap();
            let has_next = i != block_strs_max_index;
            blocks.push(Block {
                adjust_width: 0,
                has_next,
                width: calcurator.calc_block_width(block_str.to_string()),
                block_string: block_str.to_string(),
            })
        }
        Line { blocks }
    }

    fn set_adjust_width(&mut self, block_index: usize, adjust_width: usize) {
        if let Some(block) = self.blocks.get_mut(block_index) {
            block.adjust_width = adjust_width;
        }
    }
}

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
    use crate::tabstops::BlockCalcurator;
    use crate::tabstops::Lines;
    use unicode_width::UnicodeWidthStr;

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

    struct MyCalcurator {}

    impl BlockCalcurator for MyCalcurator {
        fn calc_block_width(&self, block_string: String) -> usize {
            block_string.width_cjk()
        }
        fn split_line(&self, line: String) -> Vec<String> {
            let regexp = regex::Regex::new(r"\t|\s+").unwrap();
            regexp.split(line.as_ref()).map(|s| s.to_string()).collect()
        }
        fn margin(&self) -> usize {
            1
        }
        fn tabsize(&self) -> usize {
            2
        }
    }

    #[test]
    fn test_my_simple() {
        my_assert(
            "\
var hoge\t= 123;
var mogegegegege\t= 234;
var a\t= 345;
a  h  == 1234;
",
            "\
var hoge         =  123;
var mogegegegege =  234;
var a            =  345;
a   h            == 1234;
",
        );
    }

    fn my_assert(input: &str, expect: &str) {
        assert_eq!(
            Lines::new_with_calcurator(String::from(input), Box::new(MyCalcurator {})).to_string(),
            String::from(expect)
        );
    }
}
