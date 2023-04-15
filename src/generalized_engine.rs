// use crate::dsl::Expression;
// use crate::dsl::*;
// use eyre::Result;
// use solidity::ast;
// use std::io;
// use solidity::ast::AstVisitor;

// #[derive(Clone)]
// pub struct Context {
//     pub current_source_unit: Option<ast::SourceUnit>,
//     pub current_contract_definition: Option<ast::ContractDefinition>,
//     pub current_definition_node: Option<ast::ContractDefinitionNode>,
// }

// #[derive(Clone)]
// pub struct MatchingEngine {
//     rule: Rule,
//     matches: Option<Vec<SourceLocation>>,
//     context: Context,
// }

// // Temporary fix
// // TODO properly define SourceLocation
// pub type SourceLocation = String;

// impl MatchingEngine {
//     fn new(rule: Rule, context: Context) -> Self {
//         MatchingEngine {
//             rule,
//             matches: None,
//             context
//         }
//     }
//     fn match_source_unit(&mut self, ast: &ast::SourceUnit) -> Option<Vec<SourceLocation>> {
//         let mut context = ast::SourceUnitContext {
//             // FIXME
//             source_units: &[],
//             current_source_unit: &ast
//         };
//         self.visit_source_unit(&mut context);
//         self.leave_source_unit(&mut context);
//         self.matches.clone()
//     }
//     fn match_expression(&mut self, ast: &ast::Expression) -> Option<Vec<SourceLocation>> {
//         let mut context = ast::ExpressionContext {
//             source_units: &[],
//             current_source_unit: self.context.current_source_unit.as_ref().unwrap(),
//             contract_definition: self.context.current_contract_definition.as_ref().unwrap(),
//             definition_node: self.context.current_definition_node.as_ref().unwrap(),
//             blocks: &mut vec![],
//             statement: None,
//             expression: ast,
//         };
//         self.visit_expression(&mut context);
//         // self.leave_expression(&mut context);
//         self.matches.clone()
//     }
// }

// impl MatchingEngine {
//     fn print_message(
//         &mut self,
//         contract_definition: &ast::ContractDefinition,
//         definition_node: &ast::ContractDefinitionNode,
//         source_line: usize,
//         expression: &dyn std::fmt::Display,
//     ) {
//         // TODO properly encode messages from the rule
//         println!(
//             "\t{} contains `{}` usage,
//             contract_definition.definition_node_location(source_line, definition_node)",
//             expression, "TODO",
//         );
//     }
// }

// impl ast::AstVisitor for MatchingEngine {
//     fn visit_function_call<'a, 'b>(
//         &mut self,
//         context: &mut ast::FunctionCallContext<'a, 'b>,
//     ) -> io::Result<()> {
//         let pattern = self.rule.pattern.clone();
//         let mut matches = true;
//         // For every non-empty field of the pattern, we match against the FunctionCallContext
//         match pattern {
//             Pattern::Expression(ExpressionWrapper { expression }) => {
//                 match expression {
//                     Expression::FunctionCall(function_call_rule) => {
//                         if let Some(kind) = &function_call_rule.kind {
//                             // TODO, for many fields, this should recurse
//                             matches = matches && *kind == context.function_call.kind;
//                         }
//                         if let Some(try_call) = &function_call_rule.try_call {
//                             matches = matches && *try_call == context.function_call.try_call;
//                         }
//                         // TODO implement the Regex fields
//                         if let Some(names) = &function_call_rule.names {
//                             matches = matches && *names == context.function_call.names;
//                         }
//                         // TODO implement matching checks for arguments, is_constant, is_pure, l_value_requested,
//                         // type_descriptions, src, id etc

//                         // This is the tricky one!
//                         if matches {
//                             if let Some(expression) = &function_call_rule.expression {
//                                 let mut child = self.clone();
//                                 child.matches = None;
//                                 child.rule.pattern = Pattern::Expression(ExpressionWrapper {
//                                     expression: *expression.clone(),
//                                 });

//                                 let child_matches =
//                                     child.match_expression(&context.function_call.expression);
//                                 // TODO, the location should be made more granular here
//                                 if let Some(matches) = child_matches {
//                                     self.matches = insert_source_location(&self.matches, matches);
//                                 }
//                             }
//                         }
//                     }
//                     _ => {
//                         // TODO Should we do this?
//                     }
//                 }
//             }
//         }

//         // TODO, can't we skip these?
//         // for argument in context.function_call.arguments.iter() {
//         //     let mut argument_context = ast::ExpressionContext {
//         //         source_units: context.source_units,
//         //         current_source_unit: context.current_source_unit,
//         //         contract_definition: context.contract_definition,
//         //         definition_node: context.definition_node,
//         //         blocks: context.blocks,
//         //         statement: context.statement,
//         //         expression: argument,
//         //     };

//         //     self.visit_expression(&mut argument_context)?;
//         //     // We don't want the matches to appear again, if we made a replacement
//         //     // TODO
//         //     self.leave_expression(&mut argument_context)?;
//         // }

//         Ok(())
//     }

//     fn leave_function_call<'a, 'b>(
//         &mut self,
//         context: &mut ast::FunctionCallContext<'a, 'b>,
//     ) -> io::Result<()> {
//         todo!()
//     }

//     fn visit_member_access<'a, 'b>(
//         &mut self,
//         context: &mut ast::MemberAccessContext<'a, 'b>,
//     ) -> io::Result<()> {
//         todo!()
//     }

//     fn leave_member_access<'a, 'b>(
//         &mut self,
//         context: &mut ast::MemberAccessContext<'a, 'b>,
//     ) -> io::Result<()> {
//         todo!()
//     }
// }

// fn insert_source_location(
//     matches: &Option<Vec<SourceLocation>>,
//     location: Vec<SourceLocation>,
// ) -> Option<Vec<SourceLocation>> {
//     if let Some(matches) = matches {
//         let mut new_vec: Vec<SourceLocation> = matches.clone();
//         new_vec.extend(location.iter().cloned());
//         Some(new_vec)
//     } else {
//         Some(location)
//     }
// }
