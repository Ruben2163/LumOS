use core::fmt::{self, Write};

struct File {
    name: &'static str,
    content: &'static str,
}

impl File {
    fn new(name: &'static str, content: &'static str) -> Self {
        File { name, content }
    }
}

struct FileSystem {
    files: [File; 1],
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            files: [File::new("hello.txt", "Hello from file system!")],
        }
    }

    fn read_file(&self, name: &str) -> Option<&str> {
        for file in &self.files {
            if file.name == name {
                return Some(file.content);
            }
        }
        None
    }
}