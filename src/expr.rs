use crate::error::*;
use crate::token::*;
use crate::object::*;

pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

impl Expr {
    pub fn accept<T>(&self, expr_visitor: &dyn ExprVisitor<T>) -> Result<T, CDSyntaxError> {
        match self {
            Expr::Binary(v) => v.accept(expr_visitor),
            Expr::Grouping(v) => v.accept(expr_visitor),
            Expr::Literal(v) => v.accept(expr_visitor),
            Expr::Unary(v) => v.accept(expr_visitor),
        }
    }
}

pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

pub struct LiteralExpr {
    pub value: Option<Object>,
}

pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, expr: &BinaryExpr) -> Result<T, CDSyntaxError>;
    fn visit_grouping_expr(&self, expr: &GroupingExpr) -> Result<T, CDSyntaxError>;
    fn visit_literal_expr(&self, expr: &LiteralExpr) -> Result<T, CDSyntaxError>;
    fn visit_unary_expr(&self, expr: &UnaryExpr) -> Result<T, CDSyntaxError>;
}

impl BinaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, CDSyntaxError> {
        visitor.visit_binary_expr(self)
    }
}

impl GroupingExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, CDSyntaxError> {
        visitor.visit_grouping_expr(self)
    }
}

impl LiteralExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, CDSyntaxError> {
        visitor.visit_literal_expr(self)
    }
}

impl UnaryExpr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> Result<T, CDSyntaxError> {
        visitor.visit_unary_expr(self)
    }
}

