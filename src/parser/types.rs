use log::trace;

use crate::advance_with_expected;
use crate::core::types::{SimpleType, Type};
use crate::core::{errors::SyntaxError, token::Kind};

use super::Parser;

impl Parser {
    fn parse_array_type(&mut self) -> Result<Type, Vec<SyntaxError>> {
        trace!("parsing type array");
        advance_with_expected!(
            Kind::RightSquare,
            self,
            advance_with_expected!(
                Kind::LitInt,
                self,
                advance_with_expected!(
                    Kind::LeftSquare,
                    self,
                    advance_with_expected!(Kind::Of, self, self.parse_type())
                )
            )
        )
    }

    pub fn parse_type(&mut self) -> Result<Type, Vec<SyntaxError>> {
        trace!("parsing type");
        match self.advance().kind {
            Kind::TString => Ok(Type::Simple(SimpleType::String)),
            Kind::TInt => Ok(Type::Simple(SimpleType::Int)),
            Kind::TArray => self.parse_array_type(),
            Kind::TBool => Ok(Type::Simple(SimpleType::Bool)),
            other => Err(vec![self.error_at_current(
                format!("Expected type declaration, found {}", other).as_str(),
            )]),
        }
    }
}
