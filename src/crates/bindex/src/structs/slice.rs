use color_eyre::eyre::eyre;

#[allow(unused)]
pub trait SliceExtended {
    fn read_8xU8(&self) -> color_eyre::Result<[u8; 8]>;
    fn read_be_u8(&self) -> color_eyre::Result<u8>;
    fn read_be_u16(&self) -> color_eyre::Result<u16>;
    fn read_be_u32(&self) -> color_eyre::Result<u32>;
    fn read_be_u64(&self) -> color_eyre::Result<u64>;
    fn read_exact(&self, buf: &mut [u8]) -> color_eyre::Result<()>;
}

impl SliceExtended for &[u8] {
    fn read_8xU8(&self) -> color_eyre::Result<[u8; 8]> {
        let mut buf: [u8; 8] = [0; 8];
        (&self[..8]).read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_be_u8(&self) -> color_eyre::Result<u8> {
        let mut buf: [u8; 1] = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_be_bytes(buf))
    }

    fn read_be_u16(&self) -> color_eyre::Result<u16> {
        let mut buf: [u8; 2] = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_be_bytes(buf))
    }

    fn read_be_u32(&self) -> color_eyre::Result<u32> {
        let mut buf: [u8; 4] = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_be_bytes(buf))
    }

    fn read_be_u64(&self) -> color_eyre::Result<u64> {
        let mut buf: [u8; 8] = [0; 8];
        self.read_exact(&mut buf)?;
        Ok(u64::from_be_bytes(buf))
    }

    fn read_exact(&self, buf: &mut [u8]) -> color_eyre::Result<()> {
        let buf_len = buf.len();
        if self.len() != buf_len {
            dbg!(self.len(), buf_len);
            return Err(eyre!("Not exact len"));
        }
        self.iter().take(buf_len).enumerate().for_each(|(i, r)| {
            buf[i] = *r;
        });
        Ok(())
    }
}
