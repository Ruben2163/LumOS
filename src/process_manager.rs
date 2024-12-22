struct Process {
    pid: u32,
    name: &'static str,
}

impl Process {
    fn new(pid: u32, name: &'static str) -> Self {
        Process { pid, name }
    }
}