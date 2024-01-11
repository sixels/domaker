use lalrpop_util::lalrpop_mod;

lalrpop_mod!(spec);
pub use spec::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
            module foo;

            schema Foo(max_bar) {
                /// This will be generated as doc comment
                /// for the field `bar`.
                bar: Uint8(
                    max = max_bar,
                    min = 1,
                ),
                baz: String?,
                abc: List<Int32(max=1, min = -1)>(max = 10),
                xyz: OtherSchema,
            }

            schema Bar {
                foo: Foo,
                baz: Baz,
            }

            schema Baz = String;
            schema OtherSchema = Baz;
        "#;

        // Token
        let ast = ModuleParser::new().parse(input).unwrap();
        println!("{:#?}", ast);
    }
}
