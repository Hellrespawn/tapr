use super::Value;
use crate::error::ErrorKind;
use crate::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash)]
struct ValueWrapper {
    mutable: bool,
    value: Value,
}

impl ValueWrapper {
    fn def(value: Value) -> Self {
        ValueWrapper {
            mutable: false,
            value,
        }
    }

    fn var(value: Value) -> Self {
        ValueWrapper {
            mutable: true,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    map: HashMap<String, ValueWrapper>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            parent: None,
        }
    }

    pub fn merge_values(&mut self, other: Environment) -> Result<()> {
        for (key, value) in other.map {
            self.insert(key, value)?;
        }

        Ok(())
    }

    pub fn set_parent(&mut self, environment: Environment) {
        self.parent.replace(Box::new(environment));
    }

    pub fn take_parent(&mut self) -> Option<Environment> {
        let env = self.parent.take();

        env.map(|e| *e)
    }

    pub fn def(&mut self, key: String, value: Value) -> Result<()> {
        self.insert(key, ValueWrapper::def(value))
    }

    pub fn var(&mut self, key: String, value: Value) -> Result<()> {
        self.insert(key, ValueWrapper::var(value))
    }

    fn insert(&mut self, key: String, value: ValueWrapper) -> Result<()> {
        if !value.mutable && self.has_in_scope(&key) {
            Err(ErrorKind::SymbolDefined(key).into())
        } else {
            self.map.insert(key, value);
            Ok(())
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key).map(|v| &v.value).or_else(|| {
            if let Some(environment) = &self.parent {
                environment.get(key)
            } else {
                None
            }
        })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.map.get_mut(key).map(|v| &mut v.value).or_else(|| {
            if let Some(environment) = &mut self.parent {
                environment.get_mut(key)
            } else {
                None
            }
        })
    }

    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn has_in_scope(&self, key: &str) -> bool {
        self.map.get(key).is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn format_table(mut values: Vec<(&str, &Value)>) -> String {
        values.sort_by(|(l, _), (r, _)| l.cmp(r));

        let key_width = values
            .iter()
            .map(|(k, _)| k.len())
            .max()
            .unwrap_or_default();

        let value_width = values
            .iter()
            .map(|(_, v)| v.to_string().len())
            .max()
            .unwrap_or_default();

        let mut string = format!(
            "+-{}-+-{}-+\n",
            "-".repeat(key_width),
            "-".repeat(value_width)
        );

        for (key, value) in values {
            if key.starts_with('_') {
                continue;
            }

            string += &format!(
                "| {key:>key_width$} | {:>value_width$} |\n",
                value.to_string()
            );
        }

        string += &format!(
            "+-{}-+-{}-+",
            "-".repeat(key_width),
            "-".repeat(value_width)
        );

        string
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            parent.fmt(f)?;
        }

        let values = self
            .map
            .iter()
            .map(|(k, v)| (k.as_str(), &v.value))
            .collect::<Vec<_>>();

        writeln!(f, "{}", Self::format_table(values))
    }
}

impl std::hash::Hash for Environment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.map.iter().collect::<Vec<_>>().hash(state);
        self.parent.hash(state);
    }
}
