/*
*
*/

use std::cell::RefCell;
use std::error::Error;
use std::fs;
use std::rc::{Rc, Weak};

pub enum Command {
    ListContents,
    ChangeDir,
}

pub enum TerminalLine {
    Command(Command),
    File { size: u32, name: String },
    Folder { name: String },
}

pub enum Contents {
    File { name: String, size: u32 },
    Directory(Rc<Directory>),
}

pub struct Directory {
    name: String,
    contents: RefCell<Vec<Contents>>,
    father: Option<Weak<Directory>>,
}

impl Directory {
    fn new(name: String, father: Option<&Rc<Directory>>) -> Rc<Directory> {
        let new_dir = Rc::new(Directory {
            name,
            contents: RefCell::new(Vec::new()),
            father: match father {
                Some(father) => Some(Rc::downgrade(father)),
                None => None,
            },
        });

        if let Some(f) = father {
            f.contents
                .borrow_mut()
                .push(Contents::Directory(Rc::clone(&new_dir)));
        }

        new_dir
    }

    fn add_to_contents(&mut self, content: Contents) {
        self.contents.borrow_mut().push(content);
    }
}

pub fn read_input() -> Result<Directory, Box<dyn Error>> {
    let f = fs::read_to_string("mock_input.txt").expect("input provided by problem");

    let source = Directory::new(String::from("/"), None);
    let mut current_dir = source;

    for line in f.lines() {
        let line_vec: Vec<&str> = line.split_whitespace().collect();
        //enum terminal_line seems like a better option
        match line_vec[0] {
            "$" => match line_vec[1] {
                "cd" => match line_vec[2] {
                    ".." => match current_dir.father {
                        Some(father) => {
                            current_dir = current_dir.father.unwrap().upgrade().unwrap()
                        }
                        None => (),
                    },
                    _ => current_dir.add_to_contents(Contents::Directory(Directory::new(
                        line_vec[3].to_string(),
                        Some(&current_dir),
                    ))),
                },
                "ls" => (),
            },
        }

        /*
         * $ what to do with next line ->
         *   ls = add_to_contents
         *   cd = change current Directory
         * if not a command line -> contents are being shown
         */
    }
    todo!()
}
