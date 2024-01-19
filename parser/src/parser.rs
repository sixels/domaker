use lalrpop_util::{lalrpop_mod, ParseError};

lalrpop_mod!(grammar);
use grammar::*;

use crate::{
    ast::{Location, Module},
    error::{Error, ParseErrors},
};

pub struct Parser {}

impl Parser {
    pub fn parse<'input>(
        filename: &str,
        content: &'input str,
    ) -> Result<Module<'input>, ParseErrors> {
        let mut errors = Vec::new();

        match ModuleParser::new().parse(filename.as_ref(), &mut errors, content) {
            Ok(module) if errors.is_empty() => Ok(module),
            Ok(_) => Err(ParseErrors {
                filename: filename.to_string(),
                errors: errors
                    .into_iter()
                    .map(|e| match e {
                        ParseError::InvalidToken { location } => Error::InvalidToken {
                            location: Location::new(location, location),
                        },
                    })
                    .collect(),
            }),
            Err(e) => Err(ParseErrors {
                filename: filename.to_string(),
                errors: vec![Error::InvalidToken {
                    location: Location::new(e.location, e.location),
                }],
            }),
        }
    }
}

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
        let mut errors = Vec::new();

        let ast = ModuleParser::new().parse("a", &mut errors, input).unwrap();
        println!("{:#?}", ast);
    }
}
