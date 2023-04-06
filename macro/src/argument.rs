use proc_macro2::Span;

use crate::{
    parse::{Parse, Parser},
    span_error::SpanError,
};

pub struct DuchessDeclaration {
    pub packages: Vec<JavaPackage>,
}

impl Parse for DuchessDeclaration {
    fn parse(p: &mut Parser) -> Result<Option<Self>, SpanError> {
        let packages = JavaPackage::parse_many(p)?;
        Ok(Some(DuchessDeclaration { packages }))
    }

    fn description() -> String {
        format!("list of classes whose methods you would like to call (e.g., `java.lang.Object`)")
    }
}

pub struct JavaPackage {
    pub package_name: JavaPath,
    pub classes: Vec<JavaClass>,
}

impl Parse for JavaPackage {
    fn parse(p: &mut Parser) -> Result<Option<Self>, SpanError> {
        let Some(()) = p.eat_keyword("package") else {
            return Ok(None);
        };

        let Some(package_name) = JavaPath::parse(p)? else {
            return Err(p.error("expected package name"));
        };

        let Some(()) = p.eat_punct(';') else {
            return Err(p.error("expected `;` after package name"));
        };

        let classes = JavaClass::parse_many(p)?;

        Ok(Some(JavaPackage {
            package_name,
            classes,
        }))
    }

    fn description() -> String {
        format!("java package to reflect (e.g., `package foo; ...`)")
    }
}

pub struct JavaClass {
    pub class_name: String,
    pub members: Option<Vec<String>>,
}

impl Parse for JavaClass {
    fn parse(p: &mut Parser) -> Result<Option<Self>, SpanError> {
        let Some(()) = p.eat_keyword("class") else {
            return Ok(None);
        };

        let Some(class_name) = p.eat_ident() else {
            return Err(p.error("expected class name"));
        };

        let Some(()) = p.eat_punct('{') else {
            return Err(p.error("expected '{' after class name"));
        };

        // FIXME: support a list of method names

        let Some(()) = p.eat_punct('*') else {
            return Err(p.error("expected '*'"));
        };

        let Some(()) = p.eat_punct('}') else {
            return Err(p.error("expected '}' after class methods"));
        };

        Ok(Some(JavaClass {
            class_name,
            members: None,
        }))
    }

    fn description() -> String {
        format!("java class to reflect (e.g., `class Foo {{ * }}`)")
    }
}

pub struct JavaPath {
    pub text: String,
    pub span: Span,
}

impl Parse for JavaPath {
    fn parse(p: &mut Parser) -> Result<Option<Self>, SpanError> {
        let Some(mut text) = p.eat_ident() else {
            return Ok(None);
        };

        let span = p.last_span().unwrap();

        while let Some(()) = p.eat_punct('.') {
            let Some(next) = p.eat_ident() else {
                return Err(SpanError { span: p.last_span().unwrap(), message: format!("expected identifier after `.`") });
            };
            text.push_str(&next);
        }

        Ok(Some(JavaPath { text, span }))
    }

    fn description() -> String {
        format!("java class name (e.g., `java.lang.Object`)")
    }
}
