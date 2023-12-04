use aoc_runner_derive::{aoc, aoc_generator};
use std::{cell::RefCell, rc::Rc};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|v| v.to_string()).collect()
}
type FolderRef = Rc<RefCell<Folder>>;

pub struct Folder {
    name: String,
    files: Vec<(i32, String)>,
    folders: Vec<FolderRef>,
}

impl Folder {
    pub fn new(name: String) -> Folder {
        Folder {
            name,
            files: Vec::<(i32, String)>::new(),
            folders: Vec::<FolderRef>::new(),
        }
    }

    pub fn add_file(mut self, size: i32, name: String) {
        self.files.push((size, name));
    }

    pub fn add_folder(mut self, name: String) {
        self.folders.push(Rc::new(RefCell::new(Folder::new(name))));
    }

    pub fn get_folder(self, name: String) -> Option<FolderRef> {
        self.folders.into_iter().find(|f| f.borrow().name == name)
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &Vec<String>) -> i32 {
    let mut path = Vec::<String>::new();
    let root: FolderRef = Rc::new(RefCell::new(Folder::new('/'.to_string())));
    path.push(root.borrow().name.clone());
    for line in input {
        if line.starts_with("$ cd") {
            if line.ends_with('/') {
                path = Vec::<String>::new();
                path.push(root.borrow().name.clone());
            }
            if line.ends_with("..") {
                let _ = path.pop();
            } else {
                let mut pointer = Rc::clone(&root);
                for p in &path {
                    pointer = pointer.borrow().get_folder(p.to_string()).unwrap();
                }
            }
        } else if line.starts_with("dir") {
            todo!();
        }
    }
    0
}

//#[aoc(day7, part2)]
//pub fn part2(input: &Vec<&str>) -> i32 {
//    todo!();
//}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &str = indoc! {"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 95437);
    }

    //#[test]
    //fn example_part2() {
    //    assert_eq!(part2(&input_generator(EXAMPLE)), 12);
    //}
}
