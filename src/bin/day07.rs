use std::any::Any;
use std::path::{Component, Path};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Index, IndexMut};

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
struct EntryId(usize);

struct FileSystem {
    entries: Vec<FileSystemEntry>,
}

impl FileSystem {
    fn root(&self) -> EntryId {
        EntryId(0)
    }

    fn add_entry(&mut self, parent: EntryId, name: impl Into<String>, size: Option<usize>, is_dir: bool) -> EntryId {
        let name = name.into();
        let entry_id = EntryId(self.entries.len());
        self.entries.push(FileSystemEntry {
            size,
            parent,
            is_dir,
            children: Default::default(),
        });
        self[parent].children.insert(name, entry_id);
        entry_id
    }

    fn cd(&self, current: EntryId, path: &Path) -> EntryId {
        let mut current = current;
        for component in path.components() {
            current = match component {
                Component::Prefix(_) => unimplemented!(),
                Component::RootDir => self.root(),
                Component::CurDir => current,
                Component::ParentDir => self[current].parent,
                Component::Normal(name) =>
                    match self[current].children.get(name.to_str().unwrap()) {
                        Some(entry) => *entry,
                        None => panic!("could not find entry {:?}", name.to_str())
                    }
            }
        }
        current
    }

    fn entry_size(&mut self, path: EntryId) -> usize {
        if let Some(size) = self[path].size {
            size
        } else {
            let mut size = 0;
            let tmp = self[path].children.values().copied().collect::<Vec<_>>();
            for child_entry_id in tmp {
                size += self.entry_size(child_entry_id);
            }
            self[path].size = Some(size);
            size
        }
    }

    fn dirs(&self) -> Vec<EntryId> {
        self.entries.iter().enumerate().filter_map(|(idx, entry)| entry.is_dir.then_some(EntryId(idx))).collect::<Vec<_>>()
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self {
            entries: vec![FileSystemEntry {
                parent: EntryId(0),
                children: Default::default(),
                size: None,
                is_dir: true
            }]
        }
    }
}

impl Debug for FileSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn print_node(f: &mut Formatter<'_>, fs: &FileSystem, entry_id: EntryId, depth: usize) -> std::fmt::Result{
            let entry = &fs[entry_id];
            for (name, child_entry_id) in entry.children.iter() {
                let child_entry = &fs[*child_entry_id];
                writeln!(f, "{}- {} ({}, size={:?})","  ".repeat(depth), name, if child_entry.is_dir { "dir" } else { "file" }, &child_entry.size)?;
                print_node(f, fs, *child_entry_id, depth+1)?;
            }
            Ok(())
        }
        print_node(f, self, self.root(), 0)
    }
}

#[derive(Debug)]
struct FileSystemEntry {
    parent: EntryId,
    children: HashMap<String, EntryId>,
    size: Option<usize>,
    is_dir: bool,
}

impl Index<EntryId> for FileSystem {
    type Output = FileSystemEntry;

    fn index(&self, index: EntryId) -> &Self::Output {
        &self.entries[index.0]
    }
}

impl IndexMut<EntryId> for FileSystem {
    fn index_mut(&mut self, index: EntryId) -> &mut Self::Output {
        &mut self.entries[index.0]
    }
}


fn main() {
    // Read input file
    let day = Path::new(file!()).file_stem().unwrap().to_str().unwrap();
    let binding = std::fs::read_to_string(format!("inputs/{day}.txt")).unwrap();
    let mut input = binding.as_str();

    let mut fs = FileSystem::default();
    let mut current_dir = fs.root();

    let mut iter = input.lines().peekable();
    while let Some(command) = iter.next() {
        if command.starts_with("$ ls") {
            while let Some(line) = iter.peek() {
                if line.starts_with("$") {
                    break
                }

                // Do something with next
                let line = iter.next().unwrap();
                if let Some(name) = line.strip_prefix("dir "){
                    fs.add_entry(current_dir, name.trim(), None, true);
                }
                else if let Some((size, name)) = line.split_once(char::is_whitespace) {
                    fs.add_entry(current_dir, name.trim(), Some(size.parse().unwrap()), false);
                }
                else{
                    unreachable!();
                }
            }
        } else if let Some(suffix) = command.strip_prefix("$ cd ") {
            let path = Path::new(suffix.trim());
            current_dir = fs.cd(current_dir, path);
        } else {
            unreachable!()
        }
    }
    fs.entry_size(fs.root());
    println!("{:?}", fs);

    let mut dirs_sizes: Vec<_> = fs.dirs().into_iter().map(|entry_id| fs.entry_size(entry_id)).collect();
    dirs_sizes.retain(|&entry| entry < 100000);
    let total_size:usize = dirs_sizes.iter().sum();

    println!("Solution part 1: {}", total_size);
}
