use super::builtins::get_builtin_functions;
use super::Value;
use crate::error::ErrorKind;
use crate::Result;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    map: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn root() -> Self {
        let builtins = get_builtin_functions();

        let mut map = HashMap::new();

        for builtin in builtins {
            map.insert(builtin.name().to_owned(), Value::Builtin(builtin));
        }

        Self { map, parent: None }
    }

    pub fn empty() -> Self {
        Self {
            map: HashMap::new(),
            parent: None,
        }
    }

    pub fn set_parent(&mut self, environment: Environment) {
        self.parent.replace(Box::new(environment));
    }

    pub fn take_parent(&mut self) -> Option<Environment> {
        let env = self.parent.take();

        env.map(|e| *e)
    }

    pub fn insert(&mut self, key: String, value: Value) -> Result<()> {
        if self.has_in_scope(&key) {
            Err(ErrorKind::SymbolDefined(key).into())
        } else {
            self.map.insert(key, value);
            Ok(())
        }
    }

    pub fn set(&mut self, key: String, value: Value) -> Result<()> {
        if self.has_in_scope(&key) {
            let old_value = self.map.insert(key, value);

            debug_assert!(old_value.is_some());
            Ok(())
        } else {
            Err(ErrorKind::SymbolNotDefined(key).into())
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.map.get(key).or_else(|| {
            if let Some(environment) = &self.parent {
                environment.get(key)
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

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(parent) = &self.parent {
            parent.fmt(f)?;
        }

        let mut values = self.map.iter().collect::<Vec<_>>();

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

        writeln!(
            f,
            "+-{}-+-{}-+",
            "-".repeat(key_width),
            "-".repeat(value_width)
        )?;

        for (key, value) in values {
            if key.starts_with('_') {
                continue;
            }

            writeln!(
                f,
                "| {key:>key_width$} | {:>value_width$} |",
                value.to_string()
            )?;
        }

        writeln!(
            f,
            "+-{}-+-{}-+",
            "-".repeat(key_width),
            "-".repeat(value_width)
        )?;

        Ok(())
    }
}
