use crate::Environment;

mod modules;

pub fn get_default_environment() -> Environment {
    let mut environment = Environment::new();

    for module in modules::get_modules() {
        if module.is_core_module() {
            environment
                .merge_values(module.environment())
                .unwrap_or_else(|_| {
                    panic!("Unable to merge core '{}' module.", module.name())
                });
        } else {
            environment
                .def(module.name().to_owned(), module.environment().into())
                .unwrap_or_else(|_| {
                    panic!("Unable to insert '{}' module.", module.name())
                });
        }
    }

    environment
}
