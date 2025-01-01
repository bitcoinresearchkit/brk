use std::io::Read;

pub trait SliceExtended {
    fn read_u8(&self) -> u8;
    fn read_u32(&self) -> u32;
    fn read_u64(&self) -> u64;
    fn read_exact(&self, buf: &mut [u8]);
}

impl SliceExtended for fjall::Slice {
    fn read_u8(&self) -> u8 {
        let mut buf: [u8; 1] = [0; 1];
        self.read_exact(&mut buf);
        u8::from_be_bytes(buf)
    }

    fn read_u32(&self) -> u32 {
        let mut buf: [u8; 4] = [0; 4];
        self.read_exact(&mut buf);
        u32::from_be_bytes(buf)
    }

    fn read_u64(&self) -> u64 {
        let mut buf: [u8; 8] = [0; 8];
        self.read_exact(&mut buf);
        u64::from_be_bytes(buf)
    }

    fn read_exact(&self, buf: &mut [u8]) {
        self.bytes().take(buf.len()).enumerate().for_each(|(i, r)| {
            buf[i] = r.unwrap();
        });
    }
}
