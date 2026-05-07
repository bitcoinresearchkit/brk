use brk_error::Result;
use serde_json::{Map, Value};

use crate::{fields::Ctx, mode::Mode, path::Path};

pub struct Formatter {
    mode: Mode,
    fields: Vec<Path>,
}

impl Formatter {
    pub fn new(mode: Mode, fields: Vec<Path>) -> Self {
        Self { mode, fields }
    }

    pub fn format(&self, ctx: &Ctx) -> Result<String> {
        match self.mode {
            Mode::Bare => self.bare(ctx),
            Mode::Tsv => self.tsv(ctx),
            Mode::Json => Ok(serde_json::to_string(&self.object(ctx)?)?),
            Mode::Pretty => Ok(serde_json::to_string_pretty(&self.object(ctx)?)?),
        }
    }

    fn bare(&self, ctx: &Ctx) -> Result<String> {
        let mut out = String::new();
        flatten(&ctx.resolve(&self.fields[0])?, &mut out);
        Ok(out)
    }

    fn tsv(&self, ctx: &Ctx) -> Result<String> {
        let mut row = String::new();
        for (i, path) in self.fields.iter().enumerate() {
            if i > 0 {
                row.push('\t');
            }
            for c in ctx.resolve_str(path)?.chars() {
                row.push(if matches!(c, '\t' | '\n' | '\r') { ' ' } else { c });
            }
        }
        Ok(row)
    }

    fn object(&self, ctx: &Ctx) -> Result<Value> {
        let mut obj = Map::with_capacity(self.fields.len());
        for path in &self.fields {
            obj.insert(path.raw.clone(), ctx.resolve(path)?);
        }
        Ok(Value::Object(obj))
    }
}

fn flatten(v: &Value, out: &mut String) {
    match v {
        Value::Array(arr) => arr.iter().for_each(|item| flatten(item, out)),
        Value::String(s) => push_line(out, s),
        other => push_line(out, &other.to_string()),
    }
}

fn push_line(out: &mut String, s: &str) {
    if !out.is_empty() {
        out.push('\n');
    }
    out.push_str(s);
}
