// Copyright (C) 2019-2021 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

//! The abstract syntax tree (ast) for a Leo program.
//!
//! This module contains the [`Ast`] type, a wrapper around the [`Program`] type.
//! The [`Ast`] type is intended to be parsed and modified by different passes
//! of the Leo compiler. The Leo compiler can generate a set of R1CS constraints from any [`Ast`].

// Re-enable when circleci supports rust 1.54.0.
// #![doc = include_str!("../README.md")]

pub mod annotation;
pub use self::annotation::*;

pub mod circuits;
pub use self::circuits::*;

pub mod chars;
pub use self::chars::*;

pub mod common;
pub use self::common::*;

pub mod expression;
pub use self::expression::*;

pub mod functions;
pub use self::functions::*;

pub mod groups;
pub use self::groups::*;

pub mod imports;
pub use self::imports::*;

pub mod input;
pub use self::input::*;

pub mod program;
pub use self::program::*;

pub mod reducer;
pub use self::reducer::*;

pub mod statements;
pub use self::statements::*;

pub mod types;
pub use self::types::*;

mod node;
pub use node::*;

use leo_errors::{AstError, Result};

/// The abstract syntax tree (AST) for a Leo program.
///
/// The [`Ast`] type represents a Leo program as a series of recursive data types.
/// These data types form a tree that begins from a [`Program`] type root.
///
/// A new [`Ast`] can be created from a [`Grammar`] generated by the pest parser in the `grammar` module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ast {
    ast: Program,
}

impl Ast {
    /// Creates a new AST from a given program tree.
    pub fn new(program: Program) -> Self {
        Self { ast: program }
    }

    /// Mutates the program ast by preforming canonicalization on it.
    pub fn canonicalize(&mut self) -> Result<()> {
        self.ast = ReconstructingDirector::new(Canonicalizer::default()).reduce_program(self.as_repr())?;
        Ok(())
    }

    /// Returns a reference to the inner program AST representation.
    pub fn as_repr(&self) -> &Program {
        &self.ast
    }

    pub fn into_repr(self) -> Program {
        self.ast
    }

    /// Serializes the ast into a JSON string.
    pub fn to_json_string(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.ast).map_err(|e| AstError::failed_to_convert_ast_to_json_string(&e))?)
    }

    /// Serializes the ast into a JSON file.
    pub fn to_json_file(&self, mut path: std::path::PathBuf, file_name: &str) -> Result<()> {
        path.push(file_name);
        let file = std::fs::File::create(&path).map_err(|e| AstError::failed_to_create_ast_json_file(&path, &e))?;
        let writer = std::io::BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, &self.ast)
            .map_err(|e| AstError::failed_to_write_ast_to_json_file(&path, &e))?)
    }

    /// Deserializes the JSON string into a ast.
    pub fn from_json_string(json: &str) -> Result<Self> {
        let ast: Program = serde_json::from_str(json).map_err(|e| AstError::failed_to_read_json_string_to_ast(&e))?;
        Ok(Self { ast })
    }

    /// Deserializes the JSON string into a ast from a file.
    pub fn from_json_file(path: std::path::PathBuf) -> Result<Self> {
        let data = std::fs::read_to_string(&path).map_err(|e| AstError::failed_to_read_json_file(&path, &e))?;
        Self::from_json_string(&data)
    }
}

impl AsRef<Program> for Ast {
    fn as_ref(&self) -> &Program {
        &self.ast
    }
}
