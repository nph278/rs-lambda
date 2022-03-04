#![allow(clippy::enum_glob_use)]
use crate::types::Token::{self, *};
use reglex::{rule_list, RuleList};

pub fn rules() -> RuleList<Token> {
    rule_list![
        r"\s" => |_| None,
        r"λ" => |_| Some(Lambda),
        r"\\" => |_| Some(Lambda),
        r"\(" => |_| Some(Left),
        r"\)" => |_| Some(Right),
        r"[a-zA-Z0-9]+" => |s| Some(Variable(s[0].to_string())),
    ]
}
