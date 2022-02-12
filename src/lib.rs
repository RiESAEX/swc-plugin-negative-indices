use std::collections::HashMap;

use swc_plugin::{ast::*, plugin_transform, syntax_pos::DUMMY_SP, util::take::Take};

struct MyPlugin;

impl VisitMut for MyPlugin {
    fn visit_mut_member_expr(&mut self, expr: &mut MemberExpr) {
        expr.visit_mut_children_with(self);
        match &mut expr.prop {
            MemberProp::Computed(ComputedPropName { span, expr: c_expr }) => match &mut **c_expr {
                Expr::Unary(UnaryExpr {
                    span: _,
                    op: UnaryOp::Minus,
                    arg,
                }) => match &mut **arg {
                    Expr::Lit(Lit::Num(num)) => {
                        if num.value == 0.0 {
                            return;
                        };
                        *c_expr = Expr::Bin(BinExpr {
                            span: DUMMY_SP,
                            op: BinaryOp::Sub,
                            left: Box::new(Expr::Member(MemberExpr {
                                span: DUMMY_SP,
                                obj: expr.obj.clone(),
                                prop: Ident {
                                    sym: js_word!("length"),
                                    span: DUMMY_SP,
                                    optional: false,
                                }
                                .into(),
                            })),
                            right: arg.take(),
                        })
                        .into();
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}

/// An entrypoint to the SWC's transform plugin.
/// `plugin_transform` macro handles necessary interop to communicate with the host,
/// and entrypoint function name (`process_transform`) can be anything else.
///
/// If plugin need to handle low-level ptr directly,
/// it is possible to opt out from macro by writing transform fn manually via raw interface
///
/// `__plugin_process_impl(
///     ast_ptr: *const u8,
///     ast_ptr_len: i32,
///     config_str_ptr: *const u8,
///     config_str_ptr_len: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */
///
/// However, this means plugin author need to handle all of serialization/deserialization
/// steps with communicating with host. Refer `swc_plugin_macro` for more details.

#[plugin_transform]
pub fn process_transform(program: Program, _plugin_config: String) -> Program {
    println!(
        "{:#?}",
        serde_json::from_str::<HashMap<String, String>>(&_plugin_config)
    );
    program.fold_with(&mut as_folder(MyPlugin))
}
