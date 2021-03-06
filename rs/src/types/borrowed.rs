use crate::types::ast::traits;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct BorrowedSyntax<'a> {
  _phantom: PhantomData<&'a traits::Empty>,
}

impl<'a> traits::Syntax for BorrowedSyntax<'a> {
  type Script = Script<'a>;

  type Stmt = Stmt<'a>;
  type TraceStmt = TraceStmt<'a>;
  type ExprStmt = ExprStmt<'a>;

  type Expr = Expr<'a>;
  type SeqExpr = SeqExpr<'a>;
  type AssignExpr = AssignExpr<'a>;
  type BinExpr = BinExpr<'a>;
  type StrLit = StrLit<'a>;

  type Pat = Pat<'a>;
  type MemberPat = MemberPat<'a>;
  type IdentPat = IdentPat<'a>;
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct Script<'a> {
  pub loc: (),
  pub stmts: &'a [Stmt<'a>],
}

impl<'s> traits::Script<BorrowedSyntax<'s>> for Script<'s> {
  #[cfg(not(feature = "gat"))]
  fn stmts<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = &'a Stmt<'s>> + 'a> {
    Box::new(self.stmts.iter())
  }

  #[cfg(feature = "gat")]
  type Stmts<'a> = core::slice::Iter<'a, Stmt<'a>>;

  #[cfg(feature = "gat")]
  fn stmts(&self) -> Self::Stmts<'_> {
    self.stmts.iter()
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum Stmt<'a> {
  Trace(TraceStmt<'a>),
  Expr(ExprStmt<'a>),
  SyntaxError,
}

impl<'a> traits::Stmt<BorrowedSyntax<'a>> for Stmt<'a> {
  fn cast<'b>(&'b self) -> traits::StmtCast<'b, BorrowedSyntax<'a>> {
    match self {
      Stmt::Trace(ref e) => traits::StmtCast::Trace(e),
      Stmt::Expr(ref e) => traits::StmtCast::Expr(e),
      Stmt::SyntaxError => traits::StmtCast::SyntaxError,
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct TraceStmt<'a> {
  pub loc: (),
  pub value: &'a Expr<'a>,
}

impl<'a> traits::TraceStmt<BorrowedSyntax<'a>> for TraceStmt<'a> {
  fn value(&self) -> &Expr<'a> {
    self.value
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct ExprStmt<'a> {
  pub loc: (),
  pub expr: &'a Expr<'a>,
}

impl<'a> traits::ExprStmt<BorrowedSyntax<'a>> for ExprStmt<'a> {
  fn expr(&self) -> &Expr<'a> {
    self.expr
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum Expr<'a> {
  StrLit(StrLit<'a>),
  Error,
}

impl<'a> traits::Expr<BorrowedSyntax<'a>> for Expr<'a> {
  fn cast<'b>(&'b self) -> traits::ExprCast<'b, BorrowedSyntax<'a>> {
    match self {
      Expr::StrLit(ref e) => traits::ExprCast::StrLit(e),
      Expr::Error => traits::ExprCast::Error,
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct SeqExpr<'a> {
  pub loc: (),
  pub exprs: &'a [Expr<'a>],
}

impl<'s> traits::SeqExpr<BorrowedSyntax<'s>> for SeqExpr<'s> {
  #[cfg(not(feature = "gat"))]
  fn exprs<'a>(&'a self) -> Box<dyn ExactSizeIterator<Item = &'a Expr<'s>> + 'a> {
    Box::new(self.exprs.iter())
  }

  #[cfg(feature = "gat")]
  type Iter<'a> = core::slice::Iter<'a, Expr<'a>>;

  #[cfg(feature = "gat")]
  fn exprs(&self) -> Self::Iter<'_> {
    self.exprs.iter()
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct AssignExpr<'a> {
  pub loc: (),
  pub target: &'a Pat<'a>,
  pub value: &'a Expr<'a>,
}

impl<'a> traits::AssignExpr<BorrowedSyntax<'a>> for AssignExpr<'a> {
  fn target(&self) -> &Pat<'a> {
    self.target
  }

  fn value(&self) -> &Expr<'a> {
    self.value
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct BinExpr<'a> {
  pub loc: (),
  pub left: &'a Expr<'a>,
  pub right: &'a Expr<'a>,
}

impl<'a> traits::BinExpr<BorrowedSyntax<'a>> for BinExpr<'a> {
  fn left(&self) -> &Expr<'a> {
    self.left
  }

  fn right(&self) -> &Expr<'a> {
    self.right
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct StrLit<'a> {
  pub loc: (),
  pub value: &'a str,
}

impl traits::StrLit for StrLit<'_> {
  fn value(&self) -> Cow<str> {
    Cow::Borrowed(self.value)
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum Pat<'a> {
  Member(MemberPat<'a>),
  Ident(IdentPat<'a>),
  SyntaxError,
}

impl<'a> traits::Pat<BorrowedSyntax<'a>> for Pat<'a> {
  fn cast<'b>(&'b self) -> traits::PatCast<'b, BorrowedSyntax<'a>> {
    match self {
      Pat::Member(ref e) => traits::PatCast::Member(e),
      Pat::Ident(ref e) => traits::PatCast::Ident(e),
      Pat::SyntaxError => traits::PatCast::SyntaxError,
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct MemberPat<'a> {
  pub loc: (),
  pub base: &'a Expr<'a>,
  pub key: &'a Expr<'a>,
}

impl<'a> traits::MemberPat<BorrowedSyntax<'a>> for MemberPat<'a> {
  fn base(&self) -> &Expr<'a> {
    self.base
  }

  fn key(&self) -> &Expr<'a> {
    self.key
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub struct IdentPat<'a> {
  pub loc: (),
  pub name: &'a str,
}

impl traits::IdentPat for IdentPat<'_> {
  fn name(&self) -> &str {
    self.name
  }
}

#[cfg(test)]
mod seq_expr_tests {
  use super::{Expr, SeqExpr, StrLit};
  use crate::types::ast::traits::SeqExpr as _;

  #[test]
  fn test_eq_empty() {
    let left_foo = Expr::StrLit(StrLit { loc: (), value: "foo" });
    let left_bar = Expr::StrLit(StrLit { loc: (), value: "bar" });
    let left_seq = vec![left_foo, left_bar];
    let left = SeqExpr {
      loc: (),
      exprs: &left_seq,
    };

    let right_foo = Expr::StrLit(StrLit { loc: (), value: "foo" });
    let right_bar = Expr::StrLit(StrLit { loc: (), value: "bar" });
    let right_seq = vec![right_foo, right_bar];
    let right = SeqExpr {
      loc: (),
      exprs: &right_seq,
    };

    assert_eq!(left.exprs().len(), 2);
    assert_eq!(left, right);
  }
}
