use brk_error::Error;
use brk_rpc::Client;
use brk_types::{CheckedSub, Height};

pub fn parse(s: &str, client: &Client) -> brk_error::Result<(Height, Height)> {
    let (a, b) = s.split_once("..").unwrap_or((s, s));
    let needs_tip = |p: &str| p == "tip" || p.starts_with("tip-");
    let tip = if needs_tip(a) || needs_tip(b) {
        Some(client.get_last_height()?)
    } else {
        None
    };
    let start = endpoint(a, tip)?;
    let end = endpoint(b, tip)?;
    if end < start {
        return Err(Error::Parse(format!(
            "range end {end} before start {start}"
        )));
    }
    Ok((start, end))
}

fn endpoint(s: &str, tip: Option<Height>) -> brk_error::Result<Height> {
    if s == "tip" {
        return Ok(tip.expect("tip pre-resolved when input contains 'tip'"));
    }
    if let Some(rest) = s.strip_prefix("tip-") {
        let n: u32 = rest
            .parse()
            .map_err(|_| Error::Parse(format!("bad tip offset: {s}")))?;
        let tip = tip.expect("tip pre-resolved when input contains 'tip'");
        return tip
            .checked_sub(n)
            .ok_or_else(|| Error::Parse(format!("tip-{n} underflows genesis")));
    }
    let n: u32 = s
        .parse()
        .map_err(|_| Error::Parse(format!("bad height: {s}")))?;
    Ok(Height::new(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints_preserve_tip_offsets_and_height_bounds() {
        let tip = Some(Height::new(10));
        for (value, expected) in [
            ("tip", 10),
            ("tip-3", 7),
            ("tip-10", 0),
            ("42", 42),
            ("4294967295", u32::MAX),
        ] {
            assert_eq!(*endpoint(value, tip).unwrap(), expected);
        }
        for invalid in ["tip-11", "tip-", "4294967296", "-1", "abc"] {
            assert!(endpoint(invalid, tip).is_err(), "{invalid}");
        }
    }
}
