use brk_types::Format;

#[derive(Debug)]
pub enum Output {
    Json(Value),
    CSV(String),
}

impl Output {
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(self) -> String {
        match self {
            Output::CSV(s) => s,
            Output::Json(v) => unsafe { String::from_utf8_unchecked(v.to_vec()) },
        }
    }
}

#[derive(Debug)]
pub enum Value {
    Matrix(Vec<Vec<u8>>),
    List(Vec<u8>),
}

impl Value {
    pub fn to_vec(self) -> Vec<u8> {
        match self {
            Value::List(v) => v,
            Self::Matrix(m) => {
                let total_size = m.iter().map(|v| v.len()).sum::<usize>() + m.len() - 1 + 2;
                let mut matrix = Vec::with_capacity(total_size);
                matrix.push(b'[');

                for (i, vec) in m.into_iter().enumerate() {
                    if i > 0 {
                        matrix.push(b',');
                    }
                    matrix.extend(vec);
                }
                matrix.push(b']');
                matrix
            }
        }
    }
}

impl Output {
    pub fn default(format: Format) -> Self {
        match format {
            Format::CSV => Output::CSV("".to_string()),
            Format::JSON => Output::Json(Value::List(b"[]".to_vec())),
        }
    }
}
