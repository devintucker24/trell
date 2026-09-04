---
id: natural-syntax-specification
title: Natural Trell Syntax Specification
type: concept
status: active
created: '2026-09-04'
updated: '2026-09-04'
tags:
- natural-trell
- syntax
- ebnf
- grammar
domain: core
summary: Colon + indent + end grammar, keywords, and EBNF for Natural Trell.
nodes:
- id: natural-trell-syntax
  kind: primitive
  label: Natural Trell
- id: keyword-ask
  kind: primitive
- id: keyword-when
  kind: primitive
- id: keyword-end
  kind: primitive
edges:
- from: natural-trell-syntax
  to: belief-type
  rel: depends_on
- from: keyword-ask
  to: belief-type
  rel: implements
- from: keyword-when
  to: speculative-execution
  rel: implements
- from: natural-trell-syntax
  to: keyword-end
  rel: related_to
  note: 'heal: link hard orphan'
related:
- '[[core/speculative-execution-engine]]'
- '[[core/contract-and-guard-system]]'
implements_code:
- src/lexer.rs
- src/parser.rs
agent:
  priority: critical
  read_when:
  - syntax questions
  - writing Natural Trell examples
  maintain:
  - keep EBNF aligned with parser.rs
---

# Core: Natural Trell Syntax Specification

**Natural Trell** is the human- and AI-readable syntax for the Trell programming language. It is designed to bridge domain specialists (ship captains, clinicians, risk officers) and autonomous code-generation agents by combining Python-like visual ergonomics with the physical safety of explicit `end` block delimiters.

---

## 1. Syntax Philosophy: Colon + Indentation + Explicit `end`

Many modern languages force an artificial choice between two extremes:
* **Brace syntax (`{ ... }`)**: Cluttered with visual punctuation, making high-level reasoning and prompt-driven generation brittle.
* **Significant whitespace (Python style)**: Vulnerable to silent scope slips during automated copy-pasting, multi-agent code generation, or network transmission. A single missing space can indent an emergency brake under an unrelated `if` statement.

Natural Trell uses a hybrid model:
1. **Colons (`:`)** announce block headers naturally.
2. **Indentation** visually clarifies structure.
3. **Explicit `end` keywords** seal every block as an unambiguous, physical padlock.

---

## 2. Keywords & Reserved Words

| Keyword | Category | Semantics |
| :--- | :--- | :--- |
| `model` | Declaration | Defines a model contract, temperature, token budget, and cognitive invariants. |
| `contract` | Declaration | Classic synonym for `model`. |
| `guard` | Declaration | Defines a deterministic boolean verification predicate. |
| `action` | Declaration | Defines an executable procedure or function. |
| `fn` | Declaration | Classic synonym for `action`. |
| `struct` | Declaration | Defines an aggregate compound data structure. |
| `ask` | Deliberation | Invokes a model oracle to produce a `belief<T>`. |
| `quorum` | Consensus | Evaluates $N$ independent stochastic model samples for statistical agreement. |
| `consensus`| Consensus | Classic synonym for `quorum`. |
| `require` | Reduction | Enforces a guard on a belief, providing a fallback on failure. |
| `verify` | Reduction | Classic synonym for `require`. |
| `with` | Operator | Connects a target value to a guard in a verification expression. |
| `else` | Control Flow | Fallback branch in verification, or default branch in `when` blocks. |
| `when` | Branching | Begins a speculative semantic execution block across belief branches. |
| `fork` | Branching | Classic synonym for `when`. |
| `is` | Control Flow | Connector in `when target is:`. |
| `case` | Branching | Matches a specific hypothesis pattern within a `when` block. |
| `end` | Terminator | Explicitly closes any block (`model`, `struct`, `guard`, `action`, `when`). |
| `let` | Statement | Binds an immutable variable. |
| `print` | Statement | Emits a runtime value to the execution log. |
| `assert` | Statement | Enforces a deterministic runtime truth invariant. |
| `and`, `or`, `not` | Logical | English boolean operators. |

---

## 3. Formal EBNF Grammar

```ebnf
Program        ::= ( ModelDef | StructDef | GuardDef | ActionDef )* ;

ModelDef       ::= ( "model" | "contract" ) Ident ":"
                   ( ModelField )*
                   "end" ;

ModelField     ::= ( "temperature" ":" Float )
                 | ( "budget" ":" Integer )
                 | ( ( "invariant" | "require" ) ":" "confidence" ">=" Float ) ;

StructDef      ::= "struct" Ident ":"
                   ( Ident ":" Type ( "," | ";" )? )*
                   "end" ;

GuardDef       ::= "guard" Ident "(" Ident ( ":" Type )? ")" ":"
                   Expr
                   "end" ;

ActionDef      ::= ( "action" | "fn" ) Ident ( "(" ParamList? ")" )? ( ( "->" | ":" ) Type )? ":"
                   Stmt*
                   "end" ;

Stmt           ::= LetStmt
                 | AssignStmt
                 | PrintStmt
                 | AssertStmt
                 | ReturnStmt
                 | Expr ;

LetStmt        ::= "let" Ident ( ":" Type )? "=" Expr ( ";" )? ;
PrintStmt      ::= "print" ( "(" Expr ")" | Expr ) ( ";" )? ;
AssertStmt     ::= "assert" Expr ( "," StringLiteral )? ( ";" )? ;

Expr           ::= LogicalOr ;
LogicalOr      ::= LogicalAnd ( "or" LogicalAnd )* ;
LogicalAnd     ::= Equality ( "and" Equality )* ;
Equality       ::= Comparison ( ( "==" | "!=" ) Comparison )* ;
Comparison     ::= Term ( ( "<" | "<=" | ">" | ">=" ) Term )* ;
Term           ::= Factor ( ( "+" | "-" ) Factor )* ;
Factor         ::= Unary ( ( "*" | "/" | "%" ) Unary )* ;
Unary          ::= ( "!" | "not" ) Unary | Postfix ;

Postfix        ::= Primary ( "." Ident ( "(" ArgList? ")" )? | "(" ArgList? ")" )* ;

Primary        ::= Literal
                 | Ident
                 | "(" Expr ")"
                 | AskExpr
                 | QuorumExpr
                 | RequireExpr
                 | WhenExpr
                 | ConfidenceExpr
                 | JustificationExpr ;

AskExpr        ::= "ask" Ident ( "." Ident )? ( "(" Expr ")" | Expr ) ;
QuorumExpr     ::= ( "quorum" | "consensus" ) "(" Integer "," Float ")" ":" Expr "end" ;
RequireExpr    ::= ( "require" | "verify" ) Expr "with" Ident ( "else" | "fallback" ) Expr ;
WhenExpr       ::= ( "when" | "fork" ) Expr ( "is" )? ":"
                   ( "case" Ident ( "(" Ident ")" )? ":" Stmt* )*
                   ( ( "else" | "fallback" ) ":" Stmt* )?
                   "end" ;
```

---

## 4. Comprehensive Natural Trell Canonical Example

```trell
// Maritime Collision Avoidance Under COLREGs Rule 14
model LookoutAI:
    temperature: 0.1
    budget: 1500
    require: confidence >= 0.85
end

guard ClearWaterway(action: string):
    action == "HoldCourse" or action == "VeerStarboard" or action == "ThrottleDown"
end

action main:
    print "Scanning autonomous maritime radar and optical AIS sector..."

    // Deliberation returning belief<string>
    let obstacle_assessment: belief<string> = ask LookoutAI("Container vessel detected bearing 045 relative, range 1.2 nautical miles")

    let conf = confidence obstacle_assessment
    print "Confidence:"
    print conf

    // Epistemic reduction via deterministic guard
    let safe_action: certain string = require obstacle_assessment with ClearWaterway else "ThrottleDown"

    // Speculative semantic execution with zero-latency collapse
    when safe_action is:
        case VeerStarboard:
            print "Helm Action: Rudder starboard 15 degrees. Passing port-to-port."
        case ThrottleDown:
            print "Helm Action: Reversing screw to half astern to yield right of way."
        else:
            print "Helm Action: Maintaining steady heading."
    end
end
```

---

## 5. Cross-References
* Execution semantics and state rollback: [[core/speculative-execution-engine]]
* Model contracts and invariants: [[core/contract-and-guard-system]]
* Working maritime sample file: `examples/autonomous_ship.trell`
