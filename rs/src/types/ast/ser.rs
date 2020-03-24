//! This module provides adapters implementing serde's `Serialize` trait for AST types.

use super::traits::*;
use serde::{Serialize, Serializer};

pub struct SerializeScript<'a, T: Script>(pub &'a T);

impl<T: Script> Serialize for SerializeScript<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("Script", 2)?;
    struct_serializer.serialize_field("type", "Script")?;
    struct_serializer.serialize_field("body", &SerializeScriptStmts(self.0))?;
    struct_serializer.end()
  }
}

struct SerializeScriptStmts<'a, T: Script>(pub &'a T);

impl<'a, T: Script> Serialize for SerializeScriptStmts<'a, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeSeq;
    let stmts = self.0.stmts();
    let len = match stmts.size_hint() {
      (min_len, Some(max_len)) if min_len == max_len => Some(min_len),
      _ => None,
    };
    let mut seq_serializer = serializer.serialize_seq(len)?;
    for stmt in stmts {
      seq_serializer.serialize_element(&SerializeStmt(&*stmt))?;
    }
    seq_serializer.end()
  }
}

pub struct SerializeStmt<'a, T: Stmt>(pub &'a T);

impl<T: Stmt> Serialize for SerializeStmt<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    match self.0.cast() {
      StmtCast::Break(s) => SerializeBreakStmt(&*s).serialize(serializer),
      StmtCast::Error(s) => SerializeErrorStmt(&*s).serialize(serializer),
      StmtCast::Expr(s) => SerializeExprStmt(&*s).serialize(serializer),
      StmtCast::VarStmt(s) => SerializeVarStmt(&*s).serialize(serializer),
      _ => unimplemented!(),
    }
  }
}

pub struct SerializeBreakStmt<'a, T: BreakStmt>(pub &'a T);

impl<T: BreakStmt> Serialize for SerializeBreakStmt<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("BreakStmt", 1)?;
    struct_serializer.serialize_field("type", "BreakStmt")?;
    struct_serializer.end()
  }
}

pub struct SerializeErrorStmt<'a, T: ErrorStmt>(pub &'a T);

impl<T: ErrorStmt> Serialize for SerializeErrorStmt<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("ErrorStmt", 1)?;
    struct_serializer.serialize_field("type", "ErrorStmt")?;
    struct_serializer.end()
  }
}

pub struct SerializeExprStmt<'a, T: ExprStmt>(pub &'a T);

impl<T: ExprStmt> Serialize for SerializeExprStmt<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("ExprStmt", 2)?;
    struct_serializer.serialize_field("type", "ExprStmt")?;
    let expr: &<T::Ast as Syntax>::Expr = &*self.0.expr();
    struct_serializer.serialize_field("expr", &SerializeExpr(expr))?;
    struct_serializer.end()
  }
}

pub struct SerializeVarStmt<'a, T: VarStmt>(pub &'a T);

impl<T: VarStmt> Serialize for SerializeVarStmt<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("VarStmt", 2)?;
    struct_serializer.serialize_field("type", "VarStmt")?;
    struct_serializer.serialize_field("decls", &SerializeVarStmtDecls(self.0))?;
    struct_serializer.end()
  }
}

struct SerializeVarStmtDecls<'a, T: VarStmt>(pub &'a T);

impl<'a, T: VarStmt> Serialize for SerializeVarStmtDecls<'a, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeSeq;
    let decls = self.0.decls();
    let len = match decls.size_hint() {
      (min_len, Some(max_len)) if min_len == max_len => Some(min_len),
      _ => None,
    };
    let mut seq_serializer = serializer.serialize_seq(len)?;
    for decl in decls {
      seq_serializer.serialize_element(&SerializeVarDecl(&*decl))?;
    }
    seq_serializer.end()
  }
}

pub struct SerializeVarDecl<'a, T: VarDecl>(pub &'a T);

impl<T: VarDecl> Serialize for SerializeVarDecl<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("VarDecl", 1)?;
    struct_serializer.serialize_field("type", "VarDecl")?;
    struct_serializer.end()
  }
}

pub struct SerializeExpr<'a, T: Expr>(pub &'a T);

impl<T: Expr> Serialize for SerializeExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    match self.0.cast() {
      ExprCast::Assign(e) => SerializeAssignExpr(&*e).serialize(serializer),
      ExprCast::BoolLit(e) => SerializeBoolLit(&*e).serialize(serializer),
      ExprCast::Bin(e) => SerializeBinExpr(&*e).serialize(serializer),
      ExprCast::Call(e) => SerializeCallExpr(&*e).serialize(serializer),
      ExprCast::Error(e) => SerializeErrorExpr(&*e).serialize(serializer),
      ExprCast::Ident(e) => SerializeIdentExpr(&*e).serialize(serializer),
      ExprCast::Logical(e) => SerializeLogicalExpr(&*e).serialize(serializer),
      ExprCast::NumLit(e) => SerializeNumLit(&*e).serialize(serializer),
      ExprCast::Seq(e) => SerializeSeqExpr(&*e).serialize(serializer),
      ExprCast::StrLit(e) => SerializeStrLit(&*e).serialize(serializer),
      ExprCast::Update(e) => SerializeUpdateExpr(&*e).serialize(serializer),
      ExprCast::Unary(e) => SerializeUnaryExpr(&*e).serialize(serializer),
    }
  }
}

pub struct SerializeAssignExpr<'a, T: AssignExpr>(pub &'a T);

impl<T: AssignExpr> Serialize for SerializeAssignExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("AssignExpr", 1)?;
    struct_serializer.serialize_field("type", "AssignExpr")?;
    let value: &<T::Ast as Syntax>::Expr = &*self.0.value();
    struct_serializer.serialize_field("value", &SerializeExpr(value))?;
    struct_serializer.end()
  }
}

pub struct SerializeBinExpr<'a, T: BinExpr>(pub &'a T);

impl<T: BinExpr> Serialize for SerializeBinExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("BinExpr", 4)?;
    struct_serializer.serialize_field("type", "BinExpr")?;
    struct_serializer.serialize_field("op", &self.0.op())?;
    let left: &<T::Ast as Syntax>::Expr = &*self.0.left();
    struct_serializer.serialize_field("left", &SerializeExpr(left))?;
    let right: &<T::Ast as Syntax>::Expr = &*self.0.right();
    struct_serializer.serialize_field("right", &SerializeExpr(right))?;
    struct_serializer.end()
  }
}

pub struct SerializeBoolLit<'a, T: BoolLit>(pub &'a T);

impl<T: BoolLit> Serialize for SerializeBoolLit<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("BoolLit", 2)?;
    struct_serializer.serialize_field("type", "BoolLit")?;
    struct_serializer.serialize_field("value", &self.0.value())?;
    struct_serializer.end()
  }
}

pub struct SerializeCallExpr<'a, T: CallExpr>(pub &'a T);

impl<T: CallExpr> Serialize for SerializeCallExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("CallExpr", 3)?;
    struct_serializer.serialize_field("type", "CallExpr")?;
    let callee: &<T::Ast as Syntax>::Expr = &*self.0.callee();
    struct_serializer.serialize_field("callee", &SerializeExpr(callee))?;
    struct_serializer.serialize_field("args", &SerializeCallArgs(&*self.0))?;
    struct_serializer.end()
  }
}

pub struct SerializeCallArgs<'a, T: CallExpr>(pub &'a T);

impl<T: CallExpr> Serialize for SerializeCallArgs<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeSeq;
    let args = self.0.args();
    let len = match args.size_hint() {
      (min_len, Some(max_len)) if min_len == max_len => Some(min_len),
      _ => None,
    };
    let mut seq_serializer = serializer.serialize_seq(len)?;
    for arg in args {
      seq_serializer.serialize_element(&SerializeExpr(&*arg))?;
    }
    seq_serializer.end()
  }
}

pub struct SerializeErrorExpr<'a, T: ErrorExpr>(pub &'a T);

impl<T: ErrorExpr> Serialize for SerializeErrorExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("ErrorExpr", 1)?;
    struct_serializer.serialize_field("type", "ErrorExpr")?;
    struct_serializer.end()
  }
}

pub struct SerializeIdentExpr<'a, T: IdentExpr>(pub &'a T);

impl<T: IdentExpr> Serialize for SerializeIdentExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("IdentExpr", 2)?;
    struct_serializer.serialize_field("type", "IdentExpr")?;
    struct_serializer.serialize_field("name", &self.0.name())?;
    struct_serializer.end()
  }
}

pub struct SerializeLogicalExpr<'a, T: LogicalExpr>(pub &'a T);

impl<T: LogicalExpr> Serialize for SerializeLogicalExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("LogicalExpr", 4)?;
    struct_serializer.serialize_field("type", "LogicalExpr")?;
    struct_serializer.serialize_field("op", &self.0.op())?;
    let left: &<T::Ast as Syntax>::Expr = &*self.0.left();
    struct_serializer.serialize_field("left", &SerializeExpr(left))?;
    let right: &<T::Ast as Syntax>::Expr = &*self.0.right();
    struct_serializer.serialize_field("right", &SerializeExpr(right))?;
    struct_serializer.end()
  }
}

pub struct SerializeNumLit<'a, T: NumLit>(pub &'a T);

impl<T: NumLit> Serialize for SerializeNumLit<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("NumLit", 2)?;
    struct_serializer.serialize_field("type", "NumLit")?;
    struct_serializer.serialize_field("value", &self.0.value())?;
    struct_serializer.end()
  }
}

pub struct SerializeSeqExpr<'a, T: SeqExpr>(pub &'a T);

impl<T: SeqExpr> Serialize for SerializeSeqExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("SeqExpr", 2)?;
    struct_serializer.serialize_field("type", "SeqExpr")?;
    struct_serializer.serialize_field("exprs", &SerializeSeqExprExprs(self.0))?;
    struct_serializer.end()
  }
}

struct SerializeSeqExprExprs<'a, T: SeqExpr>(pub &'a T);

impl<'a, T: SeqExpr> Serialize for SerializeSeqExprExprs<'a, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeSeq;
    let exprs = self.0.exprs();
    let len = match exprs.size_hint() {
      (min_len, Some(max_len)) if min_len == max_len => Some(min_len),
      _ => None,
    };
    let mut seq_serializer = serializer.serialize_seq(len)?;
    for expr in exprs {
      seq_serializer.serialize_element(&SerializeExpr(&*expr))?;
    }
    seq_serializer.end()
  }
}

pub struct SerializeStrLit<'a, T: StrLit>(pub &'a T);

impl<T: StrLit> Serialize for SerializeStrLit<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("StrLit", 2)?;
    struct_serializer.serialize_field("type", "StrLit")?;
    struct_serializer.serialize_field("value", &self.0.value())?;
    struct_serializer.end()
  }
}

pub struct SerializeUpdateExpr<'a, T: UpdateExpr>(pub &'a T);

impl<T: UpdateExpr> Serialize for SerializeUpdateExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("UpdateExpr", 3)?;
    struct_serializer.serialize_field("type", "UpdateExpr")?;
    struct_serializer.serialize_field("op", &self.0.op())?;
    let arg: &<T::Ast as Syntax>::Expr = &*self.0.arg();
    struct_serializer.serialize_field("arg", &SerializeExpr(arg))?;
    struct_serializer.end()
  }
}

pub struct SerializeUnaryExpr<'a, T: UnaryExpr>(pub &'a T);

impl<T: UnaryExpr> Serialize for SerializeUnaryExpr<'_, T> {
  fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<Ser::Ok, Ser::Error> {
    use serde::ser::SerializeStruct;
    let mut struct_serializer = serializer.serialize_struct("UnaryExpr", 3)?;
    struct_serializer.serialize_field("type", "UnaryExpr")?;
    struct_serializer.serialize_field("op", &self.0.op())?;
    let arg: &<T::Ast as Syntax>::Expr = &*self.0.arg();
    struct_serializer.serialize_field("arg", &SerializeExpr(arg))?;
    struct_serializer.end()
  }
}
