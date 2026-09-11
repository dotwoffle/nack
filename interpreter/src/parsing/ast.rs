use crate::lexing::Token;

pub enum ProgramUnitNode {
    Expression,
}

pub enum ExpressionSubtreeRootNode {
    SubExpression(Box<ExpressionNode>),
}

pub struct NackProgramAST {
    program_units: Vec<ProgramUnitNode>,
}

impl NackProgramAST {
    pub fn new(program_units: Vec<ProgramUnitNode>) -> Self {
        NackProgramAST { program_units }
    }
}

pub struct ExpressionNode {
    subtree_node: ExpressionSubtreeRootNode,
}

pub struct IntLiteralNode {
    token: Token,
}

pub struct BoolLiteralNode {
    token: Token,
}
