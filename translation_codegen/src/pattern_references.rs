use std::collections::BTreeSet;

use fluent_syntax::ast::{CallArguments, Expression, InlineExpression, Pattern, PatternElement};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PatternReferences {
    pub variables_in_order: Vec<String>,
    pub selector_variables: BTreeSet<String>,
    pub message_references: BTreeSet<String>,
    pub term_references: BTreeSet<String>,
}

impl PatternReferences {
    pub fn collected_from(pattern: &Pattern<&str>) -> Self {
        let mut collected_references = Self::default();
        collected_references.visit_pattern(pattern);
        collected_references
    }

    fn record_variable(&mut self, variable_name: &str) {
        if !self
            .variables_in_order
            .iter()
            .any(|known_variable| known_variable == variable_name)
        {
            self.variables_in_order.push(variable_name.to_owned());
        }
    }

    fn visit_pattern(&mut self, pattern: &Pattern<&str>) {
        for pattern_element in &pattern.elements {
            if let PatternElement::Placeable { expression } = pattern_element {
                self.visit_expression(expression);
            }
        }
    }

    fn visit_expression(&mut self, expression: &Expression<&str>) {
        match expression {
            Expression::Select { selector, variants } => {
                self.record_selector(selector);
                self.visit_inline_expression(selector);
                for select_variant in variants {
                    self.visit_pattern(&select_variant.value);
                }
            }
            Expression::Inline(inline_expression) => {
                self.visit_inline_expression(inline_expression)
            }
        }
    }

    fn record_selector(&mut self, selector: &InlineExpression<&str>) {
        match selector {
            InlineExpression::VariableReference { id } => {
                self.selector_variables.insert(id.name.to_owned());
            }
            InlineExpression::FunctionReference { arguments, .. } => {
                for positional_argument in &arguments.positional {
                    if let InlineExpression::VariableReference { id } = positional_argument {
                        self.selector_variables.insert(id.name.to_owned());
                    }
                }
            }
            _ => {}
        }
    }

    fn visit_call_arguments(&mut self, call_arguments: &CallArguments<&str>) {
        for positional_argument in &call_arguments.positional {
            self.visit_inline_expression(positional_argument);
        }
        for named_argument in &call_arguments.named {
            self.visit_inline_expression(&named_argument.value);
        }
    }

    fn visit_inline_expression(&mut self, inline_expression: &InlineExpression<&str>) {
        match inline_expression {
            InlineExpression::VariableReference { id } => self.record_variable(id.name),
            InlineExpression::FunctionReference { arguments, .. } => {
                self.visit_call_arguments(arguments)
            }
            InlineExpression::MessageReference { id, .. } => {
                self.message_references.insert(id.name.to_owned());
            }
            InlineExpression::TermReference { id, arguments, .. } => {
                self.term_references.insert(id.name.to_owned());
                if let Some(term_arguments) = arguments {
                    self.visit_call_arguments(term_arguments);
                }
            }
            InlineExpression::Placeable { expression } => self.visit_expression(expression),
            InlineExpression::StringLiteral { .. } | InlineExpression::NumberLiteral { .. } => {}
        }
    }
}
