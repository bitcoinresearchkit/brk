use brk_error::{Error, Result};
use brk_rpc::Client;
use brk_types::{CheckedSub, Height};

pub struct Selector;

impl Selector {
    pub fn parse(s: &str, client: &Client) -> Result<(Height, Height)> {
        let (start, end) = match s.split_once("..") {
            Some((a, b)) => (Self::endpoint(a, client)?, Self::endpoint(b, client)?),
            None => {
                let h = Self::endpoint(s, client)?;
                (h, h)
            }
        };
        if end < start {
            return Err(Error::Parse(format!(
                "range end {end} before start {start}"
            )));
        }
        Ok((start, end))
    }

    fn endpoint(s: &str, client: &Client) -> Result<Height> {
        if s == "tip" {
            return client.get_last_height();
        }
        if let Some(rest) = s.strip_prefix("tip-") {
            let n: u32 = rest
                .parse()
                .map_err(|_| Error::Parse(format!("bad tip offset: {s}")))?;
            let tip = client.get_last_height()?;
            return tip
                .checked_sub(n)
                .ok_or_else(|| Error::Parse(format!("tip-{n} underflows genesis")));
        }
        let n: u32 = s
            .parse()
            .map_err(|_| Error::Parse(format!("bad height: {s}")))?;
        Ok(Height::new(n))
    }
}
