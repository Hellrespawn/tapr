use crate::parser::ast::*;

pub trait Visitor<T: std::fmt::Debug> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_if_expression(&mut self, if_expression: &IfExpression) -> T;
    fn visit_while_expression(
        &mut self,
        while_expression: &WhileExpression,
    ) -> T;
    fn visit_set_expression(&mut self, set_expression: &SetExpression) -> T;
    fn visit_function_call(&mut self, function_call: &FunctionCall) -> T;
    fn visit_function_definition(
        &mut self,
        function_definition: &FunctionDefinition,
    ) -> T;
    fn visit_list(&mut self, list: &List) -> T;
    fn visit_atom(&mut self, atom: &Atom) -> T;
}
