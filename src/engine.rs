use crate::dsl::Expression;
use crate::dsl::*;
use eyre::Result;
use solidity::ast;
use solidity::ast::AstVisitor;
use std::io;

// For an MVP at the hackathon, we'll only build a specialized
// matching engine.

#[derive(Clone)]
pub struct FunctionCallMatchingEngine {
    pub rule: Rule,
}

impl FunctionCallMatchingEngine {
    pub fn new(rule: Rule) -> Self {
        FunctionCallMatchingEngine { rule }
    }

    pub fn match_source_unit(self, ast: ast::SourceUnit) -> Result<()> {
        let mut context = ast::SourceUnitContext {
            source_units: &vec![ast.clone()],
            current_source_unit: &ast,
        };
        let visitors: Vec<Box<dyn AstVisitor>> = vec![Box::new(self)];

        let mut data = ast::AstVisitorData {
            analyzed_paths: std::collections::HashSet::new(),
            visitors,
        };

        data.visit_source_unit(&mut context)?;
        data.leave_source_unit(&mut context)?;

        Ok(())
    }
}

impl AstVisitor for FunctionCallMatchingEngine {
    fn visit_function_call<'a, 'b>(
        &mut self,
        context: &mut ast::FunctionCallContext<'a, 'b>,
    ) -> io::Result<()> {
        let mut matches = true;
        match &self.rule.pattern {
            Pattern::Expression(ExpressionWrapper { expression }) => match expression.clone() {
                Expression::FunctionCall(function_call_rule) => {
                    if let Some(kind) = &function_call_rule.kind {
                        matches = matches && *kind == context.function_call.kind;
                    }
                    if let Some(is_pure) = &function_call_rule.is_pure {
                        matches = matches && *is_pure == Some(context.function_call.is_pure);
                    }
                }
                Expression::MemberAccess(_) => todo!(),
            },
        }

        if matches {
            println!("Found a match.");
            println!("\x1b[32m{:#?}\x1b[0m", context.function_call);
        }
        Ok(())
    }
}

#[test]
fn non_pure_call() -> Result<()> {
    let rule: Rule =
        serde_yaml::from_reader(std::fs::File::open("test/examples/NonPureCalls.yaml")?).unwrap();
    println!("{rule:#?}");
    let source_unit: ast::SourceUnit = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open("test/examples/raw_call.json")?,
    ))
    .unwrap();
    println!("{source_unit:#?}");
    let mut context = ast::SourceUnitContext {
        source_units: &vec![],
        current_source_unit: &source_unit,
    };
    let function_call_matching_engine = FunctionCallMatchingEngine::new(rule);
    let visitors: Vec<Box<dyn AstVisitor>> = vec![Box::new(function_call_matching_engine)];

    let mut data = ast::AstVisitorData {
        analyzed_paths: std::collections::HashSet::new(),
        visitors,
    };

    data.visit_source_unit(&mut context)?;
    data.leave_source_unit(&mut context)?;

    Ok(())
}
