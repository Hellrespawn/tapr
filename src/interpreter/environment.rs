use super::builtin::get_builtin_functions;
use super::Value;
use std::collections::HashMap;

pub struct Environment {
    map: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn root() -> Self {
        let builtins = get_builtin_functions();

        let mut map = HashMap::new();

        for function in builtins {
            map.insert(function.name().to_owned(), Value::Function(function));
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

    pub fn insert(&mut self, key: String, value: Value) -> Option<Value> {
        self.map.insert(key, value)
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

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.map.get_mut(key).or_else(|| {
            if let Some(environment) = &mut self.parent {
                environment.get_mut(key)
            } else {
                None
            }
        })
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::empty()
    }
}
