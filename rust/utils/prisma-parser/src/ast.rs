mod argument;
mod comment;
mod config;
mod expression;
mod find_at_position;
mod identifier;
mod indentation_type;
mod newline_type;
mod source_config;
mod top;
mod traits;

pub(crate) use self::comment::Comment;

pub use crate::diagnostics::Span;
pub use argument::{Argument, ArgumentsList, EmptyArgument};
pub use config::ConfigBlockProperty;
pub use expression::Expression;
pub use find_at_position::*;
pub use identifier::Identifier;
pub use indentation_type::IndentationType;
pub use newline_type::NewlineType;
pub use source_config::SourceConfig;
pub use top::Top;
pub use traits::{WithDocumentation, WithIdentifier, WithName, WithSpan};

/// AST representation of a prisma schema.
///
/// This module is used internally to represent an AST. The AST's nodes can be used
/// during validation of a schema, especially when implementing custom attributes.
///
/// The AST is not validated, also fields and attributes are not resolved. Every node is
/// annotated with its location in the text representation.
/// Basically, the AST is an object oriented representation of the schema's text.
/// Schema = Schema + Generators + Datasources
#[derive(Debug)]
pub struct SchemaAst {
    /// All models, enums, composite types, datasources, generators and type aliases.
    pub tops: Vec<Top>,
}

impl SchemaAst {
    /// Iterate over all the top-level items in the schema.
    pub fn iter_tops(&self) -> impl Iterator<Item = (TopId, &Top)> {
        self.tops
            .iter()
            .enumerate()
            .map(|(top_idx, top)| (top_idx_to_top_id(top_idx, top), top))
    }

    /// Iterate over all the datasource blocks in the schema.
    pub fn sources(&self) -> impl Iterator<Item = &SourceConfig> {
        self.tops.iter().filter_map(|top| top.as_source())
    }
}

/// An opaque identifier for a generator block in a schema AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeneratorId(u32);

/// An opaque identifier for a datasource block in a schema AST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceId(u32);

impl std::ops::Index<SourceId> for SchemaAst {
    type Output = SourceConfig;

    fn index(&self, index: SourceId) -> &Self::Output {
        self.tops[index.0 as usize].as_source().unwrap()
    }
}

/// An identifier for a top-level item in a schema AST. Use the `schema[top_id]`
/// syntax to resolve the id to an `ast::Top`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TopId {
    /// A datasource block
    Source(SourceId),
}

impl std::ops::Index<TopId> for SchemaAst {
    type Output = Top;

    fn index(&self, index: TopId) -> &Self::Output {
        let idx = match index {
            TopId::Source(SourceId(idx)) => idx,
        };

        &self.tops[idx as usize]
    }
}

fn top_idx_to_top_id(top_idx: usize, top: &Top) -> TopId {
    match top {
        Top::Source(_) => TopId::Source(SourceId(top_idx as u32)),
    }
}
