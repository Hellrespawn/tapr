#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Number(f64),
    String(String),
    Symbol(String),
    List(Vec<Self>),
}

impl Value {
    pub fn get_symbol(&self) -> Option<String> {
        if let Self::Symbol(string) = self {
            Some(string.clone())
        } else {
            None
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Number(number) => write!(f, "{number}"),
            Value::String(string) => write!(f, "\"{string}\""),
            Value::Symbol(symbol) => write!(f, "{symbol}"),
            Value::List(items) => {
                write!(f, "(")?;

                for element in items {
                    write!(f, "{element} ")?;
                }

                write!(f, "\x08)")
            }
        }
    }
}
