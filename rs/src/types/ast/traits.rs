use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Describes all the elements of a syntax.
pub trait Syntax: Sized {
  type Script: Script<Ast = Self>;

  type Stmt: Stmt<Ast = Self>;
  type BreakStmt: BreakStmt;
  /// Represents invalid statement
  type ErrorStmt: ErrorStmt;
  type ExprStmt: ExprStmt<Ast = Self>;
  type TraceStmt: TraceStmt<Ast = Self>;
  type VarDecl: VarDecl<Ast = Self>;

  type Expr: Expr<Ast = Self>;
  type AssignExpr: AssignExpr<Ast = Self>;
  type BinExpr: BinExpr<Ast = Self>;
  type BoolLit: BoolLit;
  type CallExpr: CallExpr<Ast = Self>;
  type ErrorExpr: ErrorExpr<Ast = Self>;
  type IdentExpr: IdentExpr;
  type LogicalExpr: LogicalExpr<Ast = Self>;
  type NumLit: NumLit;
  type SeqExpr: SeqExpr<Ast = Self>;
  type StrLit: StrLit;
  type UpdateExpr: UpdateExpr<Ast = Self>;
  type UnaryExpr: UnaryExpr<Ast = Self>;

  #[cfg(feature = "gat")]
  type ExprRef<'a>: core::ops::Deref<Target = Self::Expr>;

  type Pat: Pat<Ast = Self>;
  type MemberPat: MemberPat<Ast = Self>;
  type IdentPat: IdentPat;
}

/// A `Cow` variant that does not require `ToOwned`.
///
/// It is intended as a workaround until Generic Associated Types are improved.
/// Once rust-lang/rust#30472 is fixed, this type could be removed.
#[derive(Debug)]
pub enum MaybeOwned<'a, T>
where
  T: 'a,
{
  Borrowed(&'a T),
  Owned(T),
}

impl<T> std::ops::Deref for MaybeOwned<'_, T> {
  type Target = T;
  fn deref(&self) -> &T {
    match self {
      MaybeOwned::Borrowed(ref borrowed) => *borrowed,
      MaybeOwned::Owned(ref owned) => owned,
    }
  }
}

/// Script root node
pub trait Script {
  type Ast: Syntax;
  #[cfg(feature = "gat")]
  type StmtIter<'a>: Iterator<Item = MaybeOwned<'a, <Self::Ast as Syntax>::Stmt>>;

  #[cfg(feature = "gat")]
  fn stmts(&self) -> Self::StmtIter<'_>;
  #[cfg(not(feature = "gat"))]
  fn stmts<'a>(&'a self) -> Box<dyn Iterator<Item = MaybeOwned<'a, <Self::Ast as Syntax>::Stmt>> + 'a>;

  // TODO: Use the following code once rust-lang/rust#30472 is fixed.
  // #[cfg(feature = "gat")]
  // type StmtRef<'a>;
  //
  // #[cfg(feature = "gat")]
  // type Stmts<'a>: Iterator<Item = &Self::StmtRef<'a>>;
}

/// Trait representing any ActionScript statement
pub trait Stmt {
  type Ast: Syntax;

  /// Downcast the statement to its concrete type.
  fn cast(&self) -> StmtCast<Self::Ast>;
}

/// Represents the result of downcasting an expression.
pub enum StmtCast<'a, S: Syntax> {
  Break(MaybeOwned<'a, S::BreakStmt>),
  Error(MaybeOwned<'a, S::ErrorStmt>),
  Trace(MaybeOwned<'a, S::TraceStmt>),
  Expr(MaybeOwned<'a, S::ExprStmt>),
  VarDecl(MaybeOwned<'a, S::VarDecl>),
}

pub trait BreakStmt {}

pub trait ErrorStmt {}

pub trait ExprStmt {
  type Ast: Syntax;

  #[cfg(feature = "gat")]
  fn expr(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn expr<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;
}

pub trait TraceStmt {
  type Ast: Syntax;

  #[cfg(feature = "gat")]
  fn value(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn value<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;
}

/// Variable declaration statement
pub trait VarDecl {
  type Ast: Syntax;
}

// /// A single variable declarator inside a variable declaration statement
// pub trait VarDeclarator {
//   type Ast: Syntax;
//
//   #[cfg(feature = "gat")]
//   fn init(&self) -> Option<<Self::Ast as Syntax>::ExprRef<'_>>;
//   #[cfg(not(feature = "gat"))]
//   fn init<'a>(&'a self) -> Option<Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>>;
// }

/// Trait representing any ActionScript expression
pub trait Expr {
  type Ast: Syntax;

  /// Downcast the expression to its concrete type.
  fn cast(&self) -> ExprCast<Self::Ast>;
}

/// Represents the result of downcasting an expression.
pub enum ExprCast<'a, S: Syntax> {
  Assign(MaybeOwned<'a, S::AssignExpr>),
  BoolLit(MaybeOwned<'a, S::BoolLit>),
  Bin(MaybeOwned<'a, S::BinExpr>),
  Call(MaybeOwned<'a, S::CallExpr>),
  Ident(MaybeOwned<'a, S::IdentExpr>),
  Error(MaybeOwned<'a, S::ErrorExpr>),
  Logical(MaybeOwned<'a, S::LogicalExpr>),
  NumLit(MaybeOwned<'a, S::NumLit>),
  Seq(MaybeOwned<'a, S::SeqExpr>),
  StrLit(MaybeOwned<'a, S::StrLit>),
  Unary(MaybeOwned<'a, S::UnaryExpr>),
  Update(MaybeOwned<'a, S::UpdateExpr>),
}

pub trait AssignExpr {
  type Ast: Syntax;

  fn target(&self) -> &<Self::Ast as Syntax>::Pat;

  #[cfg(feature = "gat")]
  fn value(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn value<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;
}

pub trait BinExpr {
  type Ast: Syntax;

  fn op(&self) -> BinOp;

  #[cfg(feature = "gat")]
  fn left(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn left<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;

  #[cfg(feature = "gat")]
  fn right(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn right<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;
}

/// Represents all the binary operators.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum BinOp {
  /// Binary operator `+`
  #[serde(rename = "+")]
  Add,
  /// Binary operator `&`
  #[serde(rename = "&")]
  BitAnd,
  /// Binary operator `|`
  #[serde(rename = "|")]
  BitOr,
  /// Binary operator `^`
  #[serde(rename = "^")]
  BitXor,
  /// Binary operator `/`
  #[serde(rename = "/")]
  Divide,
  /// Binary operator `==`
  #[serde(rename = "==")]
  Equals,
  /// Binary operator `>`
  #[serde(rename = ">")]
  Greater,
  /// Binary operator `instanceof`
  #[serde(rename = "instanceof")]
  InstanceOf,
  /// Binary operator `add`
  #[serde(rename = "add")]
  LegacyAdd,
  /// Binary operator `<<`
  #[serde(rename = "<<")]
  LeftShift,
  /// Binary operator `<`
  #[serde(rename = "<")]
  Less,
  /// Binary operator `*`
  #[serde(rename = "*")]
  Multiply,
  /// Binary operator `!=`
  #[serde(rename = "!=")]
  NotEquals,
  /// Binary operator `!==`
  #[serde(rename = "!==")]
  NotStrictEquals,
  /// Binary operator `%`
  #[serde(rename = "%")]
  Remainder,
  /// Binary operator `>>`
  #[serde(rename = ">>")]
  SignedRightShift,
  /// Binary operator `-`
  #[serde(rename = "-")]
  Subtract,
  /// Binary operator `===`
  #[serde(rename = "===")]
  StrictEquals,
  /// Binary operator `>>>`
  #[serde(rename = ">>>")]
  UnsignedRightShift,
}

pub trait BoolLit {
  fn value(&self) -> bool;
}

pub trait CallExpr {
  type Ast: Syntax;
  #[cfg(feature = "gat")]
  type ExprIter<'a>: Iterator<Item = MaybeOwned<'a, <Self::Ast as Syntax>::Expr>>;

  #[cfg(feature = "gat")]
  fn callee(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn callee<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;

  #[cfg(feature = "gat")]
  fn args(&self) -> Self::ExprIter<'_>;
  #[cfg(not(feature = "gat"))]
  fn args<'a>(&'a self) -> Box<dyn Iterator<Item = MaybeOwned<'a, <Self::Ast as Syntax>::Expr>> + 'a>;
}

pub trait ErrorExpr {
  type Ast: Syntax;
}

pub trait IdentExpr {
  fn name(&self) -> Cow<str>;
}

pub trait LogicalExpr {
  type Ast: Syntax;

  fn op(&self) -> LogicalOp;

  #[cfg(feature = "gat")]
  fn left(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn left<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;

  #[cfg(feature = "gat")]
  fn right(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn right<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum LogicalOp {
  /// Binary operator `&&`
  #[serde(rename = "&&")]
  And,
  /// Logical operator `||`
  #[serde(rename = "||")]
  Or,
}

pub trait NumLit {
  fn value(&self) -> f64;
}

/// Sequence expression
///
/// Corresponds to two or more expressions separated by commas.
pub trait SeqExpr {
  type Ast: Syntax;
  #[cfg(feature = "gat")]
  type ExprIter<'a>: Iterator<Item = MaybeOwned<'a, <Self::Ast as Syntax>::Expr>>;

  #[cfg(feature = "gat")]
  fn exprs(&self) -> Self::ExprIter<'_>;
  #[cfg(not(feature = "gat"))]
  fn exprs<'a>(&'a self) -> Box<dyn Iterator<Item = MaybeOwned<'a, <Self::Ast as Syntax>::Expr>> + 'a>;
}

pub trait StrLit {
  fn value(&self) -> Cow<str>;
}

pub trait UnaryExpr {
  type Ast: Syntax;

  fn op(&self) -> UnaryOp;

  #[cfg(feature = "gat")]
  fn arg(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn arg<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;
}

/// Represents all the unary operators.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum UnaryOp {
  /// Prefix unary operator `~`
  #[serde(rename = "~")]
  BitNot,
  /// Prefix unary operator `!`
  #[serde(rename = "!")]
  Not,
  /// Prefix unary operator `-`
  #[serde(rename = "-")]
  Neg,
  /// Prefix unary operator `+`
  #[serde(rename = "+")]
  ToNum,
  /// Prefix unary operator `typeof`
  #[serde(rename = "typeof")]
  TypeOf,
  /// Prefix unary operator `void`
  #[serde(rename = "void")]
  Void,
}

pub trait UpdateExpr {
  type Ast: Syntax;

  fn op(&self) -> UpdateOp;

  #[cfg(feature = "gat")]
  fn arg(&self) -> <Self::Ast as Syntax>::ExprRef<'_>;
  #[cfg(not(feature = "gat"))]
  fn arg<'a>(&'a self) -> Box<dyn core::ops::Deref<Target = <Self::Ast as Syntax>::Expr> + 'a>;
}

/// Represents all the update operators.
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum UpdateOp {
  /// Prefix update operator `delete
  #[serde(rename = "delete")]
  Delete,
  /// Prefix update operator `--`
  #[serde(rename = "--_")]
  PreDec,
  /// Prefix update operator `++`
  #[serde(rename = "++_")]
  PreInc,
  /// Postfix update operator `--`
  #[serde(rename = "_--")]
  PostDec,
  /// Postfix update operator `++`
  #[serde(rename = "_++")]
  PostInc,
}

/// Trait representing any ActionScript pattern (assignment left-hand side)
pub trait Pat {
  type Ast: Syntax;

  /// Downcast the pattern to its concrete type.
  fn cast(&self) -> PatCast<Self::Ast>;
}

/// Represents the result of downcasting a pattern.
pub enum PatCast<'a, S: Syntax> {
  Member(&'a S::MemberPat),
  Ident(&'a S::IdentPat),
  SyntaxError,
}

pub trait MemberPat {
  type Ast: Syntax;

  fn base(&self) -> &<Self::Ast as Syntax>::Expr;
  fn key(&self) -> &<Self::Ast as Syntax>::Expr;
}

pub trait IdentPat {
  fn name(&self) -> &str;
}
