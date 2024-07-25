use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub struct Node {
    pub start: usize,
    pub end: usize,
}

impl Node {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug)]
#[serde(tag = "type")]
#[cfg_attr(feature = "serialize", derive(Serialize))]
pub struct Program {
    #[serde(flatten)]
    pub node: Node,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub enum Statement {
    VariableDeclarationStatement(VariableDeclaration),
    DebuggerStatement, //TODO: Might need to add some values to this enum field
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub struct DebuggerStatement {
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub struct VariableDeclaration {
    #[serde(flatten)]
    pub node: Node,
    pub declarations: Vec<VariableDeclarator>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub struct VariableDeclarator {
    #[serde(flatten)]
    pub node: Node,
    pub id: BindingIdentifier,
    pub init: Option<Expression>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
#[serde(tag = "type")]
#[cfg_attr(feature = "estree", serde(rename = "Identifier"))]
pub struct BindingIdentifier {
    #[serde(flatten)]
    pub node: Node,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub enum Expression {
    AwaitExpression(Box<AwaitExpression>), //Boxing the enums because they can be 200+ bytes. Passing them around isn't very performant. Box only passes 16 bytes
    YieldExpression(Box<YieldExpression>), //TODO: Implement memory arena later with bumpalo
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub struct AwaitExpression {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Box<Expression>
}

#[derive(Debug, Clone, Serialize, PartialEq, Hash)]
pub struct YieldExpression {
    #[serde(flatten)]
    pub node: Node,
    pub expression: Box<Expression>
}