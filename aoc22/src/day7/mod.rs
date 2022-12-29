use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
enum ElfDriveError {
    NotADirectory,
    FileNotFound,
}

enum ElfDriveObject {
    File {
        name: String,
        size: u64,
    },
    Directory {
        name: String,
        contents: HashMap<String, Rc<RefCell<ElfDriveObject>>>,
    }
}

struct ElfDirStack {
    cur: Option<Rc<RefCell<ElfDriveObject>>>,
    up: Option<Rc<ElfDirStack>>,
}

impl ElfDriveObject {
    fn is_dir(&self) -> bool {
        match self {
            Self::File { .. } => false,
            Self::Directory { .. } => true
        }
    }

    fn get_name(&self) -> &String {
        match self {
            Self::File { ref name, .. } => name,
            Self::Directory { ref name, .. } => name,
        }
    }

    fn get_size(&self) -> u64 {
        match self {
            Self::File { ref size, .. } => *size,
            Self::Directory { ref contents, .. } => {
                let mut size = 0u64;

                for ref item in contents.values() {
                    size += item.borrow().get_size();
                }

                size
            }
        }
    }

    fn get_child(&self, name: &str) -> Result<Rc<RefCell<ElfDriveObject>>, ElfDriveError> {
        match self {
            Self::File { .. } => Err(ElfDriveError::NotADirectory),
            Self::Directory { ref contents, .. } => {
                match contents.get(name) {
                    Some(o) => Ok(Rc::clone(o)),
                    None => Err(ElfDriveError::FileNotFound)
                }
            }
        }
    }

    fn insert_child(&mut self, child: &Rc<RefCell<ElfDriveObject>>) -> Result<(), ElfDriveError> {
        match self {
            Self::File { .. } => Err(ElfDriveError::NotADirectory),
            Self::Directory { ref mut contents, .. } => {
                contents.insert(child.borrow().get_name().clone(), Rc::clone(child));

                Ok(())
            }
        }
    }

    fn make_root() -> ElfDriveObject {
        Self::Directory {
            name: "".to_string(),
            contents: HashMap::new()
        }
    }
}

impl ElfDirStack {
    fn get_path(&self) -> String {
        let current = match self.cur {
            Some(ref edo) => edo.borrow().get_name().clone(),
            None => panic!("stack is empty, cannot get path")
        };

        match self.up {
            Some(ref stack) => format!("{}/{}", stack.get_path(), current),
            None => current
        }
    }

    fn of_root(root: &Rc<RefCell<ElfDriveObject>>) -> ElfDirStack {
        ElfDirStack {
            cur: Some(Rc::clone(root)),
            up: None
        }
    }

    fn of_none() -> ElfDirStack {
        ElfDirStack {
            cur: None,
            up: None
        }
    }
}

fn map_filesystem(root: &Rc<RefCell<ElfDriveObject>>, input: &String) {
    let mut pwd_stack = Rc::new(ElfDirStack::of_none());

    for line in input.lines() {
        if line.starts_with("$ ") {
            match &line["$ ".len()..] {
                s if s.starts_with("cd /") => {
                    pwd_stack = Rc::new(ElfDirStack::of_root(root));
                },
                s if s.starts_with("cd ..") => {
                    match pwd_stack.up {
                        Some(ref stack) => pwd_stack = Rc::clone(stack),
                        None => panic!("no parent directory, cannot pop")
                    };
                },
                s if s.starts_with("cd ") => {
                    let dir = match pwd_stack.cur {
                        Some(ref edo) => {
                            match edo.borrow().get_child(&s["cd ".len()..]) {
                                Ok(dir) if dir.borrow().is_dir() => dir,
                                _ => panic!("called for directory that cannot be resolved: {}", &s["cd ".len()..])
                            }
                        },
                        None => panic!("no working directory, cannot change into subdirectory")
                    };

                    pwd_stack = Rc::new(ElfDirStack {
                        cur: Some(dir),
                        up: Some(Rc::clone(&pwd_stack))
                    });
                },
                s if s == "ls" => {
                    // do nothing
                },
                _ => panic!("unsupported command: {line}")
            }

            continue;
        }

        let mut parts = line.splitn(2, ' ').peekable();

        let new_child = Rc::new(RefCell::new(if *parts.peek().unwrap() == "dir" {
            ElfDriveObject::Directory {
                name: parts.skip(1).next().unwrap().to_string(),
                contents: HashMap::new()
            }
        } else {
            ElfDriveObject::File {
                size: parts.next().unwrap().parse().unwrap(),
                name: parts.next().unwrap().to_string()
            }
        }));

        if new_child.borrow().is_dir() {
            println!("inserting  dir {}/{}/", pwd_stack.get_path(), new_child.borrow().get_name());
        } else {
            println!("inserting file {}/{}", pwd_stack.get_path(), new_child.borrow().get_name());
        }


        match pwd_stack.cur {
            Some(ref edo) => edo.borrow_mut().insert_child(&new_child).unwrap(),
            None => panic!("no working directory, cannot add new child file")
        };
    }
}

fn traverse_for_100_kibi_dirs(from: ElfDirStack, results: &mut HashMap<String, u64>) {
    let from = Rc::new(from);

    match &from.cur {
        Some(edo) => {
            match *edo.borrow() {
                ElfDriveObject::Directory { ref contents, .. } => {
                    let size = edo.borrow().get_size();

                    if size <= 100_000 {
                        results.insert(from.get_path(), size);
                    }

                    for (_key, value) in contents.iter().filter(|entry| match entry {
                        (.., value) => value.borrow().is_dir(),
                    }) {
                        traverse_for_100_kibi_dirs(ElfDirStack {
                            cur: Some(Rc::clone(value)),
                            up: Some(Rc::clone(&from))
                        }, results);
                    }
                },
                _ => panic!("expected a directory to traverse, but did not find one")
            }
        },
        None => panic!("expected a directory to traverse, but did not find one")
    }
}

fn traverse_for_viable_deletions(from: ElfDirStack, results: &mut HashMap<u64, Rc<RefCell<ElfDriveObject>>>, must_free: u64) {
    let from = Rc::new(from);

    match &from.cur {
        Some(edo) => {
            match *edo.borrow() {
                ElfDriveObject::Directory { ref contents, .. } => {
                    let size = edo.borrow().get_size();

                    if size >= must_free {
                        results.insert(size, Rc::clone(edo));
                    }

                    for (_key, value) in contents.iter().filter(|entry| match entry {
                        (.., value) => value.borrow().is_dir(),
                    }) {
                        traverse_for_viable_deletions(ElfDirStack {
                            cur: Some(Rc::clone(value)),
                            up: Some(Rc::clone(&from))
                        }, results, must_free);
                    }
                },
                _ => panic!("expected a directory to traverse, but did not find one")
            }
        },
        None => panic!("expected a directory to traverse, but did not find one")
    }
}

#[aoc(day=7, part=1)]
fn part1(input: String) -> String {
    let root = Rc::new(RefCell::new(ElfDriveObject::make_root()));

    map_filesystem(&root, &input);

    println!("/\t{}\n", root.borrow().get_size());

    let mut results: HashMap<String, u64> = HashMap::new();
    traverse_for_100_kibi_dirs(ElfDirStack::of_root(&root), &mut results);

    for (k, v) in results.iter() {
        println!("{k}\t{v}");
    }

    results.values().sum::<u64>().to_string()
}

#[aoc(day=7, part=2)]
fn part2(input: String) -> String {
    let root = Rc::new(RefCell::new(ElfDriveObject::make_root()));

    map_filesystem(&root, &input);

    const TOTAL: u64 = 70_000_000;
    const REQUIRED: u64 = 30_000_000;
    let used = root.borrow().get_size();

    if TOTAL - used >= REQUIRED {
        return format!("no need to free space: used {used} of {TOTAL}");
    }

    let mut results: HashMap<u64, Rc<RefCell<ElfDriveObject>>> = HashMap::new();
    traverse_for_viable_deletions(ElfDirStack::of_root(&root), &mut results, REQUIRED - (TOTAL - used));

    results.keys().min().unwrap().to_string()
}
