use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn chaincheck(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as syn::Stmt);
    quote! { #c }.into()
}

#[cfg(test)]
mod tests {
    use syn::{Expr, Stmt};

    use super::*;

    #[test]
    fn it_works() {
        let code = "v.iter().map(|x| x+1).filter(|x| x > 1).collect::<Vec<_>>();";

        let expr = syn::parse_str::<Stmt>(code);
        println!("{:#?}", expr);
    }
}

// let code = "v.iter().map(|x| x+1).filter(|x| x > 1);";
// -> Stmt::Expr( Expr::MethodCall (Expr::MethodCall(Expr::MethodCall)) )
// Stmt::Expr(
//     Expr::MethodCall {
//         attrs: [],
//         receiver: Expr::MethodCall {
//             attrs: [],
//             receiver: Expr::MethodCall {
//                 attrs: [],
//                 receiver: Expr::Path {
//                     attrs: [],
//                     qself: None,
//                     path: Path {
//                         leading_colon: None,
//                         segments: [
//                             PathSegment {
//                                 ident: Ident(
//                                     v,
//                                 ),
//                                 arguments: PathArguments::None,
//                             },
//                         ],
//                     },
//                 },
//                 dot_token: Dot,
//                 method: Ident(
//                     iter,
//                 ),
//                 turbofish: None,
//                 paren_token: Paren,
//                 args: [],
//             },
//             dot_token: Dot,
//             method: Ident(
//                 map,
//             ),
//             turbofish: None,
//             paren_token: Paren,
//             args: [
//                 Expr::Closure {
//                     attrs: [],
//                     lifetimes: None,
//                     constness: None,
//                     movability: None,
//                     asyncness: None,
//                     capture: None,
//                     or1_token: Or,
//                     inputs: [
//                         Pat::Ident {
//                             attrs: [],
//                             by_ref: None,
//                             mutability: None,
//                             ident: Ident(
//                                 x,
//                             ),
//                             subpat: None,
//                         },
//                     ],
//                     or2_token: Or,
//                     output: ReturnType::Default,
//                     body: Expr::Binary {
//                         attrs: [],
//                         left: Expr::Path {
//                             attrs: [],
//                             qself: None,
//                             path: Path {
//                                 leading_colon: None,
//                                 segments: [
//                                     PathSegment {
//                                         ident: Ident(
//                                             x,
//                                         ),
//                                         arguments: PathArguments::None,
//                                     },
//                                 ],
//                             },
//                         },
//                         op: BinOp::Add(
//                             Plus,
//                         ),
//                         right: Expr::Lit {
//                             attrs: [],
//                             lit: Lit::Int {
//                                 token: 1,
//                             },
//                         },
//                     },
//                 },
//             ],
//         },
//         dot_token: Dot,
//         method: Ident(
//             filter,
//         ),
//         turbofish: None,
//         paren_token: Paren,
//         args: [
//             Expr::Closure {
//                 attrs: [],
//                 lifetimes: None,
//                 constness: None,
//                 movability: None,
//                 asyncness: None,
//                 capture: None,
//                 or1_token: Or,
//                 inputs: [
//                     Pat::Ident {
//                         attrs: [],
//                         by_ref: None,
//                         mutability: None,
//                         ident: Ident(
//                             x,
//                         ),
//                         subpat: None,
//                     },
//                 ],
//                 or2_token: Or,
//                 output: ReturnType::Default,
//                 body: Expr::Binary {
//                     attrs: [],
//                     left: Expr::Path {
//                         attrs: [],
//                         qself: None,
//                         path: Path {
//                             leading_colon: None,
//                             segments: [
//                                 PathSegment {
//                                     ident: Ident(
//                                         x,
//                                     ),
//                                     arguments: PathArguments::None,
//                                 },
//                             ],
//                         },
//                     },
//                     op: BinOp::Gt(
//                         Gt,
//                     ),
//                     right: Expr::Lit {
//                         attrs: [],
//                         lit: Lit::Int {
//                             token: 1,
//                         },
//                     },
//                 },
//             },
//         ],
//     },
//     Some(
//         Semi,
//     ),
// )

// let code = "let v = v.iter().map(|x| x+1).filter(|x| x > 1).collect::<Vec<_>>();";
// -> Stmt::Local( LocalInit(Expr::MethodCall (Expr::MethodCall(Expr::MethodCall))) )
//
// Stmt::Local {
//     attrs: [],
//     let_token: Let,
//     pat: Pat::Ident {
//         attrs: [],
//         by_ref: None,
//         mutability: None,
//         ident: Ident(
//             v,
//         ),
//         subpat: None,
//     },
//     init: Some(
//         LocalInit {
//             eq_token: Eq,
//             expr: Expr::MethodCall {
//                 attrs: [],
//                 receiver: Expr::MethodCall {
//                     attrs: [],
//                     receiver: Expr::MethodCall {
//                         attrs: [],
//                         receiver: Expr::MethodCall {
//                             attrs: [],
//                             receiver: Expr::Path {
//                                 attrs: [],
//                                 qself: None,
//                                 path: Path {
//                                     leading_colon: None,
//                                     segments: [
//                                         PathSegment {
//                                             ident: Ident(
//                                                 v,
//                                             ),
//                                             arguments: PathArguments::None,
//                                         },
//                                     ],
//                                 },
//                             },
//                             dot_token: Dot,
//                             method: Ident(
//                                 iter,
//                             ),
//                             turbofish: None,
//                             paren_token: Paren,
//                             args: [],
//                         },
//                         dot_token: Dot,
//                         method: Ident(
//                             map,
//                         ),
//                         turbofish: None,
//                         paren_token: Paren,
//                         args: [
//                             Expr::Closure {
//                                 attrs: [],
//                                 lifetimes: None,
//                                 constness: None,
//                                 movability: None,
//                                 asyncness: None,
//                                 capture: None,
//                                 or1_token: Or,
//                                 inputs: [
//                                     Pat::Ident {
//                                         attrs: [],
//                                         by_ref: None,
//                                         mutability: None,
//                                         ident: Ident(
//                                             x,
//                                         ),
//                                         subpat: None,
//                                     },
//                                 ],
//                                 or2_token: Or,
//                                 output: ReturnType::Default,
//                                 body: Expr::Binary {
//                                     attrs: [],
//                                     left: Expr::Path {
//                                         attrs: [],
//                                         qself: None,
//                                         path: Path {
//                                             leading_colon: None,
//                                             segments: [
//                                                 PathSegment {
//                                                     ident: Ident(
//                                                         x,
//                                                     ),
//                                                     arguments: PathArguments::None,
//                                                 },
//                                             ],
//                                         },
//                                     },
//                                     op: BinOp::Add(
//                                         Plus,
//                                     ),
//                                     right: Expr::Lit {
//                                         attrs: [],
//                                         lit: Lit::Int {
//                                             token: 1,
//                                         },
//                                     },
//                                 },
//                             },
//                         ],
//                     },
//                     dot_token: Dot,
//                     method: Ident(
//                         filter,
//                     ),
//                     turbofish: None,
//                     paren_token: Paren,
//                     args: [
//                         Expr::Closure {
//                             attrs: [],
//                             lifetimes: None,
//                             constness: None,
//                             movability: None,
//                             asyncness: None,
//                             capture: None,
//                             or1_token: Or,
//                             inputs: [
//                                 Pat::Ident {
//                                     attrs: [],
//                                     by_ref: None,
//                                     mutability: None,
//                                     ident: Ident(
//                                         x,
//                                     ),
//                                     subpat: None,
//                                 },
//                             ],
//                             or2_token: Or,
//                             output: ReturnType::Default,
//                             body: Expr::Binary {
//                                 attrs: [],
//                                 left: Expr::Path {
//                                     attrs: [],
//                                     qself: None,
//                                     path: Path {
//                                         leading_colon: None,
//                                         segments: [
//                                             PathSegment {
//                                                 ident: Ident(
//                                                     x,
//                                                 ),
//                                                 arguments: PathArguments::None,
//                                             },
//                                         ],
//                                     },
//                                 },
//                                 op: BinOp::Gt(
//                                     Gt,
//                                 ),
//                                 right: Expr::Lit {
//                                     attrs: [],
//                                     lit: Lit::Int {
//                                         token: 1,
//                                     },
//                                 },
//                             },
//                         },
//                     ],
//                 },
//                 dot_token: Dot,
//                 method: Ident(
//                     collect,
//                 ),
//                 turbofish: Some(
//                     AngleBracketedGenericArguments {
//                         colon2_token: Some(
//                             PathSep,
//                         ),
//                         lt_token: Lt,
//                         args: [
//                             GenericArgument::Type(
//                                 Type::Path {
//                                     qself: None,
//                                     path: Path {
//                                         leading_colon: None,
//                                         segments: [
//                                             PathSegment {
//                                                 ident: Ident(
//                                                     Vec,
//                                                 ),
//                                                 arguments: PathArguments::AngleBracketed {
//                                                     colon2_token: None,
//                                                     lt_token: Lt,
//                                                     args: [
//                                                         GenericArgument::Type(
//                                                             Type::Infer {
//                                                                 underscore_token: Underscore,
//                                                             },
//                                                         ),
//                                                     ],
//                                                     gt_token: Gt,
//                                                 },
//                                             },
//                                         ],
//                                     },
//                                 },
//                             ),
//                         ],
//                         gt_token: Gt,
//                     },
//                 ),
//                 paren_token: Paren,
//                 args: [],
//             },
//             diverge: None,
//         },
//     ),
//     semi_token: Semi,
// }
