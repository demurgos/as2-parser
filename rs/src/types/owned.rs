use crate::types::ast::traits;
use serde::{Deserialize, Serialize, Serializer};
use std::borrow::Cow;

macro_rules! impl_serialize {
  ($struct_name:ident, $adapter_name:ident) => {
    impl Serialize for $struct_name {
      fn serialize<S: Serializer>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error> {
        crate::types::ast::ser::$adapter_name::<OwnedSyntax>(self).serialize(serializer)
      }
    }
  };
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
pub enum OwnedSyntax {}

impl traits::Syntax for OwnedSyntax {
  type Script = Script;

  type Stmt = Stmt;
  type BreakStmt = BreakStmt;
  type ExprStmt = ExprStmt;
  type ErrorStmt = ErrorStmt;
  type TraceStmt = TraceStmt;
  type VarDecl = VarDecl;

  type Expr = Expr;
  type AssignExpr = AssignExpr;
  type BinExpr = BinExpr;
  type BoolLit = BoolLit;
  type CallExpr = CallExpr;
  type ErrorExpr = ErrorExpr;
  type IdentExpr = IdentExpr;
  type LogicalExpr = LogicalExpr;
  type NumLit = NumLit;
  type SeqExpr = SeqExpr;
  type StrLit = StrLit;
  type UpdateExpr = UpdateExpr;
  type UnaryExpr = UnaryExpr;

  #[cfg(feature = "gat")]
  type ExprRef<'r> = &'r Expr;

  type Pat = Pat;
  type MemberPat = MemberPat;
  type IdentPat = IdentPat;
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct Script {
  pub loc: (),
  pub stmts: Vec<Stmt>,
}

impl_serialize!(Script, SerializeScript);

impl traits::Script for Script {
  type Ast = OwnedSyntax;

  #[allow(clippy::type_complexity)]
  #[cfg(feature = "gat")]
  type StmtIter<'a> = core::iter::Map<core::slice::Iter<'a, Stmt>, for<'r> fn(&'r Stmt) -> traits::MaybeOwned<'r, Stmt>>;

  #[cfg(feature = "gat")]
  fn stmts(&self) -> Self::StmtIter<'_> {
    self.stmts.iter().map(|s| traits::MaybeOwned::Borrowed(s))
  }
  #[cfg(not(feature = "gat"))]
  fn stmts<'a>(&'a self) -> Box<dyn Iterator<Item = traits::MaybeOwned<'a, Stmt>> + 'a> {
    Box::new(self.stmts.iter().map(|stmt| traits::MaybeOwned::Borrowed(stmt)))
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub enum Stmt {
  Break(BreakStmt),
  Error(ErrorStmt),
  Expr(ExprStmt),
  /// Abstract Trace Statement
  ///
  /// ```aas2
  /// @trace("Hello, World!");
  /// ```
  Trace(TraceStmt),
}

impl traits::Stmt for Stmt {
  type Ast = OwnedSyntax;

  fn cast(&self) -> traits::StmtCast<OwnedSyntax> {
    match self {
      Stmt::Break(ref s) => traits::StmtCast::Break(traits::MaybeOwned::Borrowed(s)),
      Stmt::Error(ref s) => traits::StmtCast::Error(traits::MaybeOwned::Borrowed(s)),
      Stmt::Expr(ref s) => traits::StmtCast::Expr(traits::MaybeOwned::Borrowed(s)),
      Stmt::Trace(ref s) => traits::StmtCast::Trace(traits::MaybeOwned::Borrowed(s)),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct BreakStmt {
  pub loc: (),
}

impl traits::BreakStmt for BreakStmt {}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct ErrorStmt {
  pub loc: (),
}

impl traits::ErrorStmt for ErrorStmt {}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct ExprStmt {
  pub loc: (),
  pub expr: Box<Expr>,
}

impl ExprStmt {
  fn _expr(&self) -> &Expr {
    &self.expr
  }
}

impl traits::ExprStmt for ExprStmt {
  type Ast = OwnedSyntax;

  maybe_gat_accessor!(expr, _expr, ref Expr, ref Expr);
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct TraceStmt {
  pub loc: (),
  pub value: Box<Expr>,
}

impl TraceStmt {
  fn _value(&self) -> &Expr {
    &self.value
  }
}

impl traits::TraceStmt for TraceStmt {
  type Ast = OwnedSyntax;
  maybe_gat_accessor!(value, _value, ref Expr, ref Expr);
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct VarDecl {
  pub loc: (),
}

impl traits::VarDecl for VarDecl {
  type Ast = OwnedSyntax;
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub enum Expr {
  Assign(AssignExpr),
  Bin(BinExpr),
  Error(ErrorExpr),
  Logical(LogicalExpr),
  Seq(SeqExpr),
  StrLit(StrLit),
}

impl traits::Expr for Expr {
  type Ast = OwnedSyntax;

  fn cast(&self) -> traits::ExprCast<OwnedSyntax> {
    match self {
      Expr::Seq(ref e) => traits::ExprCast::Seq(traits::MaybeOwned::Borrowed(e)),
      Expr::StrLit(ref e) => traits::ExprCast::StrLit(traits::MaybeOwned::Borrowed(e)),
      _ => unimplemented!(),
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct AssignExpr {
  pub loc: (),
  pub target: Box<Pat>,
  pub value: Box<Expr>,
}

impl AssignExpr {
  fn _value(&self) -> &Expr {
    &self.value
  }
}

impl traits::AssignExpr for AssignExpr {
  type Ast = OwnedSyntax;
  fn target(&self) -> &Pat {
    &self.target
  }

  maybe_gat_accessor!(value, _value, ref Expr, ref Expr);
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct BinExpr {
  pub loc: (),
  pub op: traits::BinOp,
  pub left: Box<Expr>,
  pub right: Box<Expr>,
}

impl BinExpr {
  fn _left(&self) -> &Expr {
    &self.left
  }

  fn _right(&self) -> &Expr {
    &self.right
  }
}

impl traits::BinExpr for BinExpr {
  type Ast = OwnedSyntax;

  fn op(&self) -> traits::BinOp {
    self.op
  }

  maybe_gat_accessor!(left, _left, ref Expr, ref Expr);
  maybe_gat_accessor!(right, _right, ref Expr, ref Expr);
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct BoolLit {
  pub loc: (),
  pub value: bool,
}

impl traits::BoolLit for BoolLit {
  fn value(&self) -> bool {
    self.value
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct CallExpr {
  pub loc: (),
  pub callee: Box<Expr>,
  pub args: Vec<Expr>,
}

impl CallExpr {
  fn _callee(&self) -> &Expr {
    &self.callee
  }
}

impl traits::CallExpr for CallExpr {
  type Ast = OwnedSyntax;
  #[allow(clippy::type_complexity)]
  #[cfg(feature = "gat")]
  type ExprIter<'a> =
    core::iter::Map<core::slice::Iter<'a, Expr>, for<'r> fn(&'r Expr) -> traits::MaybeOwned<'r, Expr>>;

  maybe_gat_accessor!(callee, _callee, ref Expr, ref Expr);

  #[cfg(feature = "gat")]
  fn args(&self) -> Self::ExprIter<'_> {
    self.args.iter().map(|e| traits::MaybeOwned::Borrowed(e))
  }
  #[cfg(not(feature = "gat"))]
  fn args<'a>(&'a self) -> Box<dyn Iterator<Item = traits::MaybeOwned<'a, Expr>> + 'a> {
    Box::new(self.args.iter().map(|e| traits::MaybeOwned::Borrowed(e)))
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct ErrorExpr {
  pub loc: (),
}

impl traits::ErrorExpr for ErrorExpr {
  type Ast = OwnedSyntax;
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct IdentExpr {
  pub loc: (),
  pub name: String,
}

impl traits::IdentExpr for IdentExpr {
  fn name(&self) -> Cow<str> {
    Cow::Borrowed(&self.name)
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct LogicalExpr {
  pub loc: (),
  pub op: traits::LogicalOp,
  pub left: Box<Expr>,
  pub right: Box<Expr>,
}

impl LogicalExpr {
  fn _left(&self) -> &Expr {
    &self.left
  }

  fn _right(&self) -> &Expr {
    &self.right
  }
}

impl traits::LogicalExpr for LogicalExpr {
  type Ast = OwnedSyntax;

  fn op(&self) -> traits::LogicalOp {
    self.op
  }

  maybe_gat_accessor!(left, _left, ref Expr, ref Expr);
  maybe_gat_accessor!(right, _right, ref Expr, ref Expr);
}

#[derive(Debug, Clone, Deserialize)]
pub struct NumLit {
  pub loc: (),
  pub value: f64,
}

impl core::cmp::PartialEq for NumLit {
  #[allow(clippy::unit_cmp)]
  fn eq(&self, other: &Self) -> bool {
    self.loc == other.loc && self.value.to_ne_bytes() == other.value.to_ne_bytes()
  }
}

impl core::cmp::Eq for NumLit {}

impl traits::NumLit for NumLit {
  fn value(&self) -> f64 {
    self.value
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct SeqExpr {
  pub loc: (),
  pub exprs: Vec<Expr>,
}

impl traits::SeqExpr for SeqExpr {
  type Ast = OwnedSyntax;
  #[allow(clippy::type_complexity)]
  #[cfg(feature = "gat")]
  type ExprIter<'a> = core::iter::Map<core::slice::Iter<'a, Expr>, for<'r> fn(&'r Expr) -> traits::MaybeOwned<'r, Expr>>;

  #[cfg(feature = "gat")]
  fn exprs(&self) -> Self::ExprIter<'_> {
    self.exprs.iter().map(|e| traits::MaybeOwned::Borrowed(e))
  }
  #[cfg(not(feature = "gat"))]
  fn exprs<'a>(&'a self) -> Box<dyn Iterator<Item = traits::MaybeOwned<'a, Expr>> + 'a> {
    Box::new(self.exprs.iter().map(|e| traits::MaybeOwned::Borrowed(e)))
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct StrLit {
  pub loc: (),
  pub value: String,
}

impl traits::StrLit for StrLit {
  fn value(&self) -> Cow<str> {
    Cow::Borrowed(&self.value)
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct UnaryExpr {
  pub loc: (),
  pub op: traits::UnaryOp,
  pub arg: Box<Expr>,
}

impl UnaryExpr {
  fn _arg(&self) -> &Expr {
    &self.arg
  }
}

impl traits::UnaryExpr for UnaryExpr {
  type Ast = OwnedSyntax;

  fn op(&self) -> traits::UnaryOp {
    self.op
  }

  maybe_gat_accessor!(arg, _arg, ref Expr, ref Expr);
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct UpdateExpr {
  pub loc: (),
  pub op: traits::UpdateOp,
  pub arg: Box<Expr>,
}

impl UpdateExpr {
  fn _arg(&self) -> &Expr {
    &self.arg
  }
}

impl traits::UpdateExpr for UpdateExpr {
  type Ast = OwnedSyntax;

  fn op(&self) -> traits::UpdateOp {
    self.op
  }

  maybe_gat_accessor!(arg, _arg, ref Expr, ref Expr);
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub enum Pat {
  MemberPat(MemberPat),
  IdentPat(IdentPat),
  SyntaxError,
}

impl traits::Pat for Pat {
  type Ast = OwnedSyntax;

  fn cast(&self) -> traits::PatCast<OwnedSyntax> {
    match self {
      Pat::MemberPat(ref e) => traits::PatCast::Member(e),
      Pat::IdentPat(ref e) => traits::PatCast::Ident(e),
      Pat::SyntaxError => traits::PatCast::SyntaxError,
    }
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct MemberPat {
  pub loc: (),
  pub base: Box<Expr>,
  pub key: Box<Expr>,
}

impl traits::MemberPat for MemberPat {
  type Ast = OwnedSyntax;

  fn base(&self) -> &Expr {
    &self.base
  }

  fn key(&self) -> &Expr {
    &self.key
  }
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Hash, Deserialize)]
pub struct IdentPat {
  pub loc: (),
  pub name: String,
}

impl traits::IdentPat for IdentPat {
  fn name(&self) -> &str {
    &self.name
  }
}

#[cfg(test)]
mod seq_expr_tests {
  use super::{Expr, SeqExpr, StrLit};
  use crate::types::ast::traits::SeqExpr as _;

  #[test]
  fn test_eq_empty() {
    let left: SeqExpr = SeqExpr {
      loc: (),
      exprs: vec![
        Expr::StrLit(StrLit {
          loc: (),
          value: String::from("foo"),
        }),
        Expr::StrLit(StrLit {
          loc: (),
          value: String::from("bar"),
        }),
      ],
    };

    let right: SeqExpr = SeqExpr {
      loc: (),
      exprs: vec![
        Expr::StrLit(StrLit {
          loc: (),
          value: String::from("foo"),
        }),
        Expr::StrLit(StrLit {
          loc: (),
          value: String::from("bar"),
        }),
      ],
    };

    assert_eq!(left.exprs().size_hint(), (2, Some(2)));
    assert_eq!(left, right);
  }
}

#[cfg(test)]
mod script_tests {
  use super::{BreakStmt, Script, Stmt};

  #[test]
  fn test_serialize() {
    let script: Script = Script {
      loc: (),
      stmts: vec![Stmt::Break(BreakStmt { loc: () }), Stmt::Break(BreakStmt { loc: () })],
    };

    let actual_json = serde_json::to_string(&script).unwrap();
    let expected_json = String::from("{\"body\":[{\"type\":\"BreakStmt\"},{\"type\":\"BreakStmt\"}]}");

    assert_eq!(actual_json, expected_json);
  }
}
