use unicode_width::UnicodeWidthStr;

use std::fmt::Write;

#[derive(Debug)]
pub struct TabstopsLines {
    pub lines: Vec<TabstopsLine>,
}

#[derive(Debug)]
pub struct Group {
    pub depth: usize,
    pub start: usize,
    pub end: usize,
    pub width: usize,
}

impl TabstopsLines {
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
        let mut current_max_width: usize = 4;

        for (i, line) in self.lines.iter().enumerate() {
            let tab_break_line = match line.blocks.get(depth) {
                Option::None => true,
                Option::Some(block) if block.block_string == "" => true,
                Option::Some(block) => {
                    if block.has_next && current_max_width < block.width {
                        current_max_width = block.width;
                    }
                    false
                }
            };
            if tab_break_line {
                start.map(|start| {
                    end.map(|end| {
                        let group = Group {
                            depth: depth,
                            start: start,
                            end: end,
                            width: current_max_width,
                        };
                        group_tuples.push(group);
                    })
                });
                start = Option::None;
                end = Option::None;
                current_max_width = 4;
            }
            if let Option::None = start {
                start = Option::Some(i);
            }
            end = Option::Some(i);
        }
        start.map(|start| {
            end.map(|end| {
                let group = Group {
                    depth: depth,
                    start: start,
                    end: end,
                    width: current_max_width,
                };
                group_tuples.push(group);
            })
        });

        group_tuples
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
                    indent = block.adjust_width + 1
                )
                .unwrap();
            }
            writeln!(result).unwrap();
        }
        result
    }
}

#[derive(Debug)]
pub struct TabstopsLine {
    pub blocks: Vec<TabstopsBlock>,
}

impl TabstopsLine {
    fn set_adjust_width(&mut self, block_index: usize, adjust_width: usize) {
        self.blocks
            .get_mut(block_index)
            .map(|block| block.adjust_width = adjust_width);
    }
}

#[derive(Debug)]
pub struct TabstopsBlock {
    pub adjust_width: usize,
    pub width: usize,
    pub has_next: bool,
    pub block_string: String,
}

pub fn parse_tabstops_lines(source: String) -> TabstopsLines {
    let lines: Vec<TabstopsLine> = source
        .lines()
        .map(|line| {
            let block_strs: Vec<String> = line.split("\t").map(|block| block.to_string()).collect();
            let mut blocks = Vec::new();
            for i in 0..block_strs.len() {
                let block_str = block_strs.get(i).unwrap();
                blocks.push(TabstopsBlock {
                    adjust_width: 0,
                    has_next: i != block_strs.len() - 1,
                    width: block_str.width_cjk(),
                    block_string: block_str.to_string(),
                })
            }
            TabstopsLine { blocks: blocks }
        })
        .collect();
    let mut tabstopsLines = TabstopsLines { lines: lines };

    let mut groups = Vec::new();
    for i in 0..tabstopsLines.max_depth() {
        groups.append(&mut tabstopsLines.groups(i));
    }
    tabstopsLines.update_width(groups);
    tabstopsLines
}

#[cfg(test)]
mod tests {
    use crate::tabstops::parse_tabstops_lines;
    #[test]
    fn test_parse_tabstops_lines() {
        let test_str = "var hoge\t= 123;\nvar mogegegegege\t= 234;\nvar a\t= 345;".to_string();
        println!("{}", test_str);
        println!("{}", parse_tabstops_lines(test_str).to_string());
    }
}
