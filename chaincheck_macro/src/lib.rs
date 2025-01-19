#[cfg(test)]
mod tests {
    use syn::{Expr, Stmt};

    use super::*;

    #[test]
    fn it_works() {
        let code = "\
#[chaincheck]
v.iter()
 .map(|x| x+1)
 .filter(|x| x.is_positive());\
";

        let expr = syn::parse_str::<Stmt>(code);
        println!("{:#?}", expr);
    }
}

// -> Stmt::Expr( Expr::MethodCall (Expr::MethodCall(Expr::MethodCall)) )

// Stmt::Expr(
//     Expr::MethodCall {
//         attrs: [
//             Attribute {
//                 pound_token: Pound,
//                 style: AttrStyle::Outer,
//                 bracket_token: Bracket,
//                 meta: Meta::Path {
//                     leading_colon: None,
//                     segments: [
//                         PathSegment {
//                             ident: Ident(
//                                 chaincheck,
//                             ),
//                             arguments: PathArguments::None,
//                         },
//                     ],
//                 },
//             },
//         ],
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
//                 body: Expr::MethodCall {
//                     attrs: [],
//                     receiver: Expr::Path {
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
//                     dot_token: Dot,
//                     method: Ident(
//                         is_positive,
//                     ),
//                     turbofish: None,
//                     paren_token: Paren,
//                     args: [],
//                 },
//             },
//         ],
//     },
//     Some(
//         Semi,
//     ),
// )
