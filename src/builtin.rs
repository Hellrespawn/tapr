use phf::phf_map;

pub type BuiltinFunction = fn(args: &[String]) -> String;

static BUILTINS: phf::Map<&'static str, BuiltinFunction> = phf_map! {
    "print" => print
};

pub fn get_builtin_function(name: &str) -> Option<&BuiltinFunction> {
    BUILTINS.get(name)
}

fn print(args: &[String]) -> String {
    args.join("")
}
