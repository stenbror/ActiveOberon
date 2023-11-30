
// ActiveOberon Compiler, a native ARM v8 & X86-64 compiler / linker / builder utility.
// Written by Richard Magnor Stenbro. Licensed under GPL v3
// Parser module for syntax analyzing of source files

use crate::scanner::{ Scanner, Symbols };

#[derive()]
pub enum Node {
	Empty
}

pub trait ParserMethods {
	fn new(scanner: Box<Scanner>) -> Parser;
}

pub trait ExpressionRules {
	fn parse_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_range_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_simple_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_term(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_factor(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_unary_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_primary_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_designator_operator(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_expression_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_index_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_array(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_set(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
}

pub trait StatementRules {
	fn parse_statement(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_case(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_statement_block(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_element_sequence(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
}

pub trait BlockRules {
	fn parse_module(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_template_parameters(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_template_parameter(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_import_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_declaration_sequence(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_constant_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_constant_expression(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_variable_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_variable_name_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_variable_name(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_flags(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_flag(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_procedure_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_operator_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_formal_parameters(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_parameter_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_body(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_type_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_array_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_math_array_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_math_array_size(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_record_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_pointer_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_procedure_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_object_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_enumeration_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_cell_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_port_list(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_port_declaration(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_port_type(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_qualified_identifier(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
	fn parse_identifier_definition(&mut self) -> Result<Box<Node>, Box<std::string::String>>;
}

/// Parser component for ActiveOberon language grammar
pub struct Parser {
	lexer: Box<Scanner>,		/* Lexical analyzer for sourcecode, returning symbols to parser rules */
	symbol: Symbols				/* Current symbol being handled in parser rule */
}

impl ParserMethods for Parser {
	fn new(scanner: Box<Scanner>) -> Parser {
		Parser {
			lexer: scanner,
			symbol: Symbols::Empty
		}
	}
}

/// Implements all expression rules in grammar of ActiveOberon
impl ExpressionRules for Parser {
	fn parse_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_range_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_simple_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_term(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_factor(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_unary_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_primary_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_designator_operator(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_expression_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_index_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_array(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_set(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}
}

/// Implements all statement rules in grammar of ActiveOberon
impl StatementRules for Parser {
	fn parse_statement(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_case(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_statement_block(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_element_sequence(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}
}

/// Implements all block rules in grammar of ActiveOberon
impl BlockRules for Parser {
	fn parse_module(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_template_parameters(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_template_parameter(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_import_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_declaration_sequence(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_constant_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_constant_expression(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_variable_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_variable_name_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_variable_name(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_flags(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_flag(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_procedure_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_operator_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_formal_parameters(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_parameter_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_body(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_type_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_array_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_math_array_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_math_array_size(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_record_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_pointer_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_procedure_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_object_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_enumeration_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_cell_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_port_list(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_port_declaration(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_port_type(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_qualified_identifier(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}

	fn parse_identifier_definition(&mut self) -> Result<Box<Node>, Box<String>> {
		todo!()
	}
}

// Unittests for parser rules

#[cfg(test)]
mod tests {

}