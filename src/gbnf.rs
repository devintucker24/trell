/// GBNF grammar for constrained decoding of Trell.
///
/// This is the unique LLM lever: a model can be forced to emit only programs
/// this checker will accept as *syntax*. Capability errors remain the compiler's job.
pub fn gbnf() -> String {
    GRAMMAR.trim_start().to_string()
}

pub const GRAMMAR: &str = r#"
# Trell — capability-checked agent programs
# Compatible with llama.cpp GBNF / xgrammar-style constrained decoding.
#
# Invalid Trell cannot be sampled. Illegal *authority* can still be sampled
# (a program that writes without `allow write`); `trell check` refuses those.

root ::= ws program ws

program ::= cap-block? in-decl* stmt*

cap-block ::= "cap" ws ident? ws "{" ws cap-item* "}" ws
cap-item ::= allow-item | deny-item | need-item | budget-item | spawn-item
allow-item ::= "allow" ws ident ws string-lit* ws
deny-item ::= "deny" ws ident ws
need-item ::= "need" ws "approve" ws "on" ws ident ws
budget-item ::= "budget" ws ident ws integer ws
spawn-item ::= "spawn" ws integer ws

in-decl ::= "in" ws ident ws ":" ws type ws

stmt ::= let-stmt | return-stmt | approve-stmt | send-stmt | expr
let-stmt ::= "let" ws ident ws "=" ws expr
return-stmt ::= "return" ws expr
approve-stmt ::= "approve" ws expr
send-stmt ::= "send" ws expr

expr ::= or-expr
or-expr ::= and-expr (ws "||" ws and-expr)*
and-expr ::= eq-expr (ws "&&" ws eq-expr)*
eq-expr ::= rel-expr (ws eq-op ws rel-expr)*
eq-op ::= "==" | "!="
rel-expr ::= add-expr (ws rel-op ws add-expr)?
rel-op ::= "<=" | ">=" | "<" | ">"
add-expr ::= mul-expr (ws add-op ws mul-expr)*
add-op ::= "+" | "-"
mul-expr ::= unary (ws mul-op ws unary)*
mul-op ::= "*" | "/"
unary ::= "-" ws unary | "!" ws unary | postfix
postfix ::= primary ("." ident)*
primary ::= integer | string-lit | "true" | "false" | ident | "(" ws expr ws ")" | if-expr | ask-expr | read-expr | write-expr | spawn-expr | record-expr

if-expr ::= "if" ws expr ws block (ws "else" ws (if-expr | block))?
block ::= "{" ws stmt* "}"
ask-expr ::= "ask" ws string-lit ws ("using" ws expr ws)? "as" ws schema
read-expr ::= "read" ws expr
write-expr ::= "write" ws unary ws expr
spawn-expr ::= "spawn" ws expr
record-expr ::= "{" ws (field (ws "," ws field)* ws)? "}"
field ::= ident ws ":" ws expr
schema ::= "{" ws (schema-field (ws "," ws schema-field)* ws)? "}"
schema-field ::= ident ws ":" ws type

type ::= "int" | "text" | "bool" | enum-type | schema
enum-type ::= "enum" ws "(" ws ident (ws "," ws ident)* ws ")"

ident ::= [a-zA-Z_] [a-zA-Z0-9_]*
integer ::= [0-9]+
string-lit ::= "\"" string-char* "\""
string-char ::= [^"\\\n] | "\\" ["\\nt]
ws ::= [ \t\n]*
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grammar_has_core_productions() {
        let g = gbnf();
        for needle in [
            "root ::=",
            "cap-block",
            "ask-expr",
            "need-item",
            "spawn-expr",
        ] {
            assert!(g.contains(needle), "missing {needle}");
        }
    }
}
