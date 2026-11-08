use std::collections::{BTreeMap, BTreeSet};

use fluent_syntax::ast::{
    CallArguments, Expression, InlineExpression, Pattern, PatternElement, VariantKey,
};

const PLURAL_CATEGORIES: [&str; 6] = ["zero", "one", "two", "few", "many", "other"];
const NUMBER_FUNCTION_NAME: &str = "NUMBER";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableKind {
    PlainText,
    CountedNumber,
    NamedChoice(BTreeSet<String>),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PatternReferences {
    pub variables_in_order: Vec<String>,
    pub selector_keys: BTreeMap<String, BTreeSet<String>>,
    pub number_function_variables: BTreeSet<String>,
    pub message_references: BTreeSet<String>,
    pub term_references: BTreeSet<String>,
}

pub fn is_plural_key(variant_key: &str) -> bool {
    PLURAL_CATEGORIES.contains(&variant_key) || variant_key.parse::<f64>().is_ok()
}

impl PatternReferences {
    pub fn collected_from(pattern: &Pattern<&str>) -> Self {
        let mut collected_references = Self::default();
        collected_references.visit_pattern(pattern);
        collected_references
    }

    pub fn merge(&mut self, other_references: Self) {
        for variable_name in other_references.variables_in_order {
            self.record_variable(&variable_name);
        }
        for (variable_name, variant_keys) in other_references.selector_keys {
            self.selector_keys
                .entry(variable_name)
                .or_default()
                .extend(variant_keys);
        }
        self.number_function_variables
            .extend(other_references.number_function_variables);
        self.message_references
            .extend(other_references.message_references);
        self.term_references
            .extend(other_references.term_references);
    }

    pub fn variable_kind(&self, variable_name: &str) -> VariableKind {
        match self.selector_keys.get(variable_name) {
            Some(variant_keys)
                if variant_keys
                    .iter()
                    .any(|variant_key| !is_plural_key(variant_key)) =>
            {
                VariableKind::NamedChoice(variant_keys.clone())
            }
            Some(_) => VariableKind::CountedNumber,
            None if self.number_function_variables.contains(variable_name) => {
                VariableKind::CountedNumber
            }
            None => VariableKind::PlainText,
        }
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
                let variant_keys = variants
                    .iter()
                    .map(|select_variant| match &select_variant.key {
                        VariantKey::Identifier { name } => (*name).to_owned(),
                        VariantKey::NumberLiteral { value } => (*value).to_owned(),
                    })
                    .collect::<BTreeSet<_>>();
                if let Some(selector_variable) = selected_variable(selector) {
                    self.selector_keys
                        .entry(selector_variable.to_owned())
                        .or_default()
                        .extend(variant_keys);
                }
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
            InlineExpression::FunctionReference { id, arguments } => {
                if id.name == NUMBER_FUNCTION_NAME {
                    for positional_argument in &arguments.positional {
                        if let InlineExpression::VariableReference { id } = positional_argument {
                            self.number_function_variables.insert(id.name.to_owned());
                        }
                    }
                }
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

fn selected_variable<'source>(selector: &InlineExpression<&'source str>) -> Option<&'source str> {
    match selector {
        InlineExpression::VariableReference { id } => Some(id.name),
        InlineExpression::FunctionReference { arguments, .. } => arguments
            .positional
            .iter()
            .find_map(|positional_argument| match positional_argument {
                InlineExpression::VariableReference { id } => Some(id.name),
                _ => None,
            }),
        _ => None,
    }
}
