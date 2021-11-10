use rocklang::compiler::{Compile, Compiler};
use rocklang::expression::{Assignment, Expression, FuncCall, FuncDecl};
use rocklang::parser::{Param, Program, Type};

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn it_compiles_numeric_asignment() {
    let program = Program {
        body: vec![Expression::Assignment(Assignment {
            left: Box::new(Expression::Identifier("x".to_string())),
            right: Box::new(Expression::Numeric(5.0)),
        })],
    };

    let mut compiler = Compiler::new(program);
    compiler.no_opt();
    compiler.compile().unwrap();

    assert_eq!(
        remove_whitespace(&compiler.ir_string()),
        remove_whitespace(
            "
		; ModuleID = 'main'\nsource_filename = \"main\"
		target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"

		define void @__main__() {
			entry:
			%x = alloca double, align 8
			store double 5.000000e+00, double* %x , align 8
			ret void
		}
	"
        )
    );
}

#[test]
fn it_compiles_numeric_to_numeric_asignment() {
    let program = Program {
        body: vec![
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("x".to_string())),
                right: Box::new(Expression::Numeric(5.0)),
            }),
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("y".to_string())),
                right: Box::new(Expression::Identifier("x".to_string())),
            }),
        ],
    };

    let mut compiler = Compiler::new(program);
    compiler.no_opt();
    compiler.compile().unwrap();

    assert_eq!(
        remove_whitespace(&compiler.ir_string()),
        remove_whitespace(
            "
		; ModuleID = 'main'\nsource_filename = \"main\"
		target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"
		define void @__main__() {
			entry:
			%x = alloca double, align 8
			store double 5.000000e+00, double* %x , align 8
			%0 = loaddouble, double* %x, align 8
			%y = alloca double, align 8
			store double %0, double* %y, align 8
			ret void
		}
	"
        )
    );
}

#[test]
fn it_compiles_new_vec_being_passed_as_fun_arg() {
    let program = Program {
        body: vec![
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("f".to_string())),
                right: Box::new(Expression::FuncDecl(FuncDecl {
                    body: vec![],
                    return_type: Type::Null,
                    params: vec![Param {
                        name: "v".to_string(),
                        typ: Type::Vector,
                    }],
                })),
            }),
            Expression::FuncCall(FuncCall {
                calee: Box::new(Expression::Identifier("f".to_string())),
                args: vec![Expression::FuncCall(FuncCall {
                    calee: Box::new(Expression::Identifier("vecnew".to_string())),
                    args: vec![],
                })],
            }),
        ],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}

#[test]
fn it_compiles_new_vec_being_passed_as_variable() {
    let program = Program {
        body: vec![
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("f".to_string())),
                right: Box::new(Expression::FuncDecl(FuncDecl {
                    body: vec![],
                    return_type: Type::Null,
                    params: vec![Param {
                        name: "v".to_string(),
                        typ: Type::Vector,
                    }],
                })),
            }),
            Expression::Assignment(Assignment {
                left: Box::new(Expression::Identifier("vecinvar".to_string())),
                right: Box::new(Expression::FuncCall(FuncCall {
                    calee: Box::new(Expression::Identifier("vecnew".to_string())),
                    args: vec![],
                })),
            }),
            Expression::FuncCall(FuncCall {
                calee: Box::new(Expression::Identifier("f".to_string())),
                args: vec![Expression::Identifier("vecinvar".to_string())],
            }),
        ],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}

#[test]
fn it_compiles_recursive_fun() {
    let program = Program {
        body: vec![Expression::Assignment(Assignment {
            left: Box::new(Expression::Identifier("f".to_string())),
            right: Box::new(Expression::FuncDecl(FuncDecl {
                return_type: Type::Null,
                params: vec![],
                body: vec![Expression::FuncCall(FuncCall {
                    calee: Box::new(Expression::Identifier("f".to_string())),
                    args: vec![],
                })],
            })),
        })],
    };

    let mut compiler = Compiler::new(program);
    compiler.compile().unwrap();
}