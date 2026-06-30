pub struct Memory {
    data: Vec<u8>,
    next: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
            next: 0,
        }
    }

    pub fn alloc(&mut self, size: usize) -> Option<usize> {
        if self.next + size > self.data.len() {
            return None;
        }
        let addr = self.next;
        self.next += size;
        Some(addr)
    }

    pub fn write(&mut self, addr: usize, bytes: &[u8]) -> bool {
        if addr + bytes.len() > self.data.len() {
            return false;
        }

        self.data[addr..addr + bytes.len()].copy_from_slice(bytes);
        true
    }

    pub fn read(&self, addr: usize, size: usize) -> Option<&[u8]> {
        if addr + size > self.data.len() {
            return None;
        }
        Some(&self.data[addr..addr + size])
    }
}
