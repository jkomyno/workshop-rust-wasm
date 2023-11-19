use crate::ast::{self, top_idx_to_top_id, traits::*};

impl ast::SchemaAst {
    /// Find the AST node at the given position (byte offset).
    pub fn find_at_position(&self, position: usize) -> SchemaPosition<'_> {
        self.find_top_at_position(position)
            .map(|top_id| match top_id {
                ast::TopId::Source(source_id) => SchemaPosition::DataSource(
                    source_id,
                    SourcePosition::new(&self[source_id], position),
                ),
                // Falling back to TopLevel as "not implemented"
                _ => SchemaPosition::TopLevel,
            })
            // If no top matched, we're in between top-level items. This is normal and expected.
            .unwrap_or(SchemaPosition::TopLevel)
    }

    /// Do a binary search for the `Top` at the given byte offset.
    pub fn find_top_at_position(&self, position: usize) -> Option<ast::TopId> {
        use std::cmp::Ordering;

        let top_idx = self.tops.binary_search_by(|top| {
            let span = top.span();

            if span.start > position {
                Ordering::Greater
            } else if span.end < position {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        top_idx
            .map(|idx| top_idx_to_top_id(idx, &self.tops[idx]))
            .ok()
    }
}

/// A cursor position in a schema.
#[derive(Debug)]
pub enum SchemaPosition<'ast> {
    /// In-between top-level items
    TopLevel,
    /// In a datasource
    DataSource(ast::SourceId, SourcePosition<'ast>),
}

#[derive(Debug)]
pub enum ExpressionPosition<'ast> {
    Expression,
    Value(&'ast str),
    Function(&'ast str),
    FunctionArgument(&'ast str, &'ast str),
}

impl<'ast> ExpressionPosition<'ast> {
    fn new(expr: &'ast ast::Expression, position: usize) -> Self {
        match expr {
            ast::Expression::StringValue(val, span) if span.contains(position) => Self::Value(val),
            ast::Expression::Function(name, args, span) if span.contains(position) => {
                let mut spans: Vec<(Option<&str>, ast::Span)> = args
                    .arguments
                    .iter()
                    .map(|arg| (arg.name.as_ref().map(|n| n.name.as_str()), arg.span()))
                    .chain(
                        args.empty_arguments
                            .iter()
                            .map(|arg| (Some(arg.name.name.as_str()), arg.name.span())),
                    )
                    .collect();

                spans.sort_by_key(|(_, span)| span.start);

                let mut arg_name = None;
                for (name, _) in spans.iter().take_while(|(_, span)| span.start < position) {
                    arg_name = Some(*name);
                }

                // If the cursor is after a trailing comma, we're not in an argument.
                if let Some(span) = args.trailing_comma {
                    if position > span.start {
                        arg_name = None;
                    }
                }

                if let Some(arg_name) = arg_name.flatten() {
                    Self::FunctionArgument(name, arg_name)
                } else {
                    Self::Function(name)
                }
            }
            _ => Self::Expression,
        }
    }
}

#[derive(Debug)]
pub enum SourcePosition<'ast> {
    /// In the general datasource
    Source,
    /// In a property
    Property(&'ast str, PropertyPosition<'ast>),
    /// Outside of the braces
    Outer,
}

impl<'ast> SourcePosition<'ast> {
    fn new(source: &'ast ast::SourceConfig, position: usize) -> Self {
        for property in &source.properties {
            if property.span.contains(position) {
                return SourcePosition::Property(
                    &property.name.name,
                    PropertyPosition::new(property, position),
                );
            }
        }

        if source.inner_span.contains(position) {
            return SourcePosition::Source;
        }

        SourcePosition::Outer
    }
}

#[derive(Debug)]
pub enum PropertyPosition<'ast> {
    /// prop
    Property,
    ///
    Value(&'ast str),
    ///
    FunctionValue(&'ast str),
}

impl<'ast> PropertyPosition<'ast> {
    fn new(property: &'ast ast::ConfigBlockProperty, position: usize) -> Self {
        if let Some(val) = &property.value {
            if val.span().contains(position) && val.is_function() {
                let func = val.as_function().unwrap();

                if func.0 == "env" {
                    return PropertyPosition::FunctionValue("env");
                }
            }
        }
        if property.span.contains(position) && !property.name.span.contains(position) {
            return PropertyPosition::Value(&property.name.name);
        }

        PropertyPosition::Property
    }
}
