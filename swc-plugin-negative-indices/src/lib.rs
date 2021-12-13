use serde::Deserialize;
use swc_atoms::js_word;
use swc_common::util::take::Take;
use swc_common::DUMMY_SP;
use swc_ecmascript::ast::{BinExpr, BinaryOp, Expr, Ident, Lit, Number, UnaryExpr, UnaryOp, ExprOrSuper};
use swc_ecmascript::visit::{Fold, VisitMut};
use swc_ecmascript::{ast::MemberExpr, visit::as_folder};
use swc_plugin::define_js_plugin;
define_js_plugin!(my_plugin);
fn my_plugin(_: Config) -> impl VisitMut + Fold {
    as_folder(MyPlugin)
}

#[derive(Debug, Deserialize)]
struct Config {}

struct MyPlugin;

impl VisitMut for MyPlugin {
    fn visit_mut_member_expr(&mut self, expr: &mut MemberExpr) {
        match &mut *expr.prop {
            Expr::Unary(UnaryExpr {
                span,
                op: UnaryOp::Minus,
                arg,
            }) => {
                match &mut **arg {
                    Expr::Lit(Lit::Num(num)) => {
                        if num.value == 0.0 {
                            return;
                        };
                        *expr.prop = Expr::Bin(BinExpr {
                            span: DUMMY_SP,
                            op: BinaryOp::Sub,
                            left: Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP,
                                obj: expr.obj.clone(),
                                prop: Box::new(Expr::Ident(Ident {
                                    sym: js_word!("length"),
                                    span: DUMMY_SP,
                                    optional: false,
                                })),
                                computed: false,
                            })),
                            right: arg.take(),
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
