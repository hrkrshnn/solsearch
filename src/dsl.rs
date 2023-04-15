use eth_lang_utils::ast::*;
use eyre::Result;
use serde::{Deserialize, Serialize};
use serde_yaml;
use solidity::ast::*;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Rule {
    id: String,
    message: String,
    metadata: Metadata,
    pattern: Pattern,
    // TODO convert to enum
    severity: String,
}

// Manually define the Expression template
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Expression {
    FunctionCall(FunctionCallRule),
    MemberAccess(MemberAccessRule),
}

impl Default for Expression {
    fn default() -> Self {
        Expression::FunctionCall(FunctionCallRule::default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "nodeType")]
pub enum Pattern {
    Expression(ExpressionWrapper),
}

// ChatGPT told me this is how it's done
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ExpressionWrapper {
    expression: Expression,
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern::Expression(ExpressionWrapper {
            expression: Expression::default(),
        })
    }
}

// TODO Maybe some macro magic here?
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCallRule {
    pub kind: Option<FunctionCallKind>,
    pub try_call: Option<Option<bool>>,
    pub names: Option<Vec<String>>,
    pub arguments: Option<Vec<String>>,
    pub expression: Option<Box<Expression>>,
    pub argument_types: Option<Option<Vec<TypeDescriptions>>>,
    pub is_constant: Option<Option<bool>>,
    pub is_pure: Option<Option<bool>>,
    pub l_value_requested: Option<bool>,
    pub type_descriptions: Option<TypeDescriptions>,
    pub src: Option<String>,
    pub id: Option<NodeID>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MemberAccessRule {
    pub member_name: Option<String>,
    pub expression: Option<Box<Expression>>,
    pub referenced_declaration: Option<Option<NodeID>>,
    pub argument_types: Option<Option<Vec<TypeDescriptions>>>,
    pub is_constant: Option<bool>,
    pub is_l_value: Option<bool>,
    pub is_pure: Option<bool>,
    pub l_value_requested: Option<bool>,
    pub type_descriptions: Option<TypeDescriptions>,
    pub src: Option<String>,
    pub id: Option<NodeID>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Metadata {
    references: Vec<String>,
    // TODO an enum of categories
    category: String,
    tags: Vec<String>,
}

#[test]
fn serialize() -> Result<()> {
    let rule = Rule::default();
    let out = serde_yaml::to_string(&rule)?;
    println!("{out}");
    Ok(())
}

#[test]
fn deserialize() -> Result<()> {
    let example = "test/examples/LowLevelCall.yaml";
    let file = File::open(example)?;
    let _rule: Rule = serde_yaml::from_reader(file)?;

    // TODO write a visitor pattern for matching memberName

    Ok(())
}
