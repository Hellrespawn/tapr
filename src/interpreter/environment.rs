use crate::error::ErrorKind;
use crate::{Node, Result};
use delegate::delegate;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Value {
    mutable: bool,
    node: Node,
}

impl Value {
    delegate! {
        to self.node {
            #[inline]
            fn to_string(&self) -> String;
        }
    }

    fn def(node: Node) -> Self {
        Value {
            mutable: false,
            node,
        }
    }

    fn var(node: Node) -> Self {
        Value {
            mutable: true,
            node,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Environment {
    map: HashMap<String, Value>,
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

    pub fn def(&mut self, key: String, node: Node) -> Result<()> {
        self.insert(key, Value::def(node))
    }

    pub fn var(&mut self, key: String, node: Node) -> Result<()> {
        self.insert(key, Value::var(node))
    }

    fn insert(&mut self, key: String, value: Value) -> Result<()> {
        if !value.mutable && self.has_in_scope(&key) {
            Err(ErrorKind::SymbolDefined(key).into())
        } else {
            self.map.insert(key, value);
            Ok(())
        }
    }

    pub fn get(&self, key: &str) -> Option<&Node> {
        self.map.get(key).map(|v| &v.node).or_else(|| {
            if let Some(environment) = &self.parent {
                environment.get(key)
            } else {
                None
            }
        })
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Node> {
        self.map.get_mut(key).map(|v| &mut v.node).or_else(|| {
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

    pub fn format_table(mut values: Vec<(&str, &Node)>) -> String {
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
            .map(|(k, v)| (k.as_str(), &v.node))
            .collect::<Vec<_>>();

        writeln!(f, "{}", Self::format_table(values))
    }
}
