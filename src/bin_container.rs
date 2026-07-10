use crate::bin_math::{cal_u8, increment_address};

pub struct Memory {
    data: Vec<u8>,
    next: u8,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        assert!(size <= 256);

        Self {
            data: vec![0; size],
            next: 0,
        }
    }

    pub fn alloc(&mut self, size: u8) -> Option<u8> {
        let (end, carry) = cal_u8(self.next, size, 0);

        if carry != 0 || end as usize > self.data.len() {
            return None;
        }

        let addr = self.next;
        self.next = end;
        Some(addr)
    }

    pub fn write(&mut self, mut addr: u8, bytes: &[u8]) -> bool {
        for byte in bytes {
            if addr as usize >= self.data.len() {
                return false;
            }

            self.data[addr as usize] = *byte;
            addr = increment_address(addr);
        }

        true
    }

    pub fn read(&self, mut addr: u8, size: u8) -> Option<Vec<u8>> {
        let mut result = Vec::new();

        for _ in 0..size {
            if addr as usize >= self.data.len() {
                return None;
            }

            result.push(self.data[addr as usize]);
            addr = increment_address(addr);
        }

        Some(result)
    }
}
