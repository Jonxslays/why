# Why Syntax

## Literals

- Char: 'a' | 'b' | 'c' | '1'| '2' | ...
- Int: 1 | 2 | ...
- Float: 1.1 | 2.3 | ...
- String: '"' Char '"'
- Boolean: true | false
- Parentheses: ( | )
- Brackets: [ | ]
- Braces: { | }
- Null: NULL

---

## Keywords

"for" | "in" | "if" | "else" | "is" | "break" | "return" | "let" | "const"

---

## Statement

Simple | If | For | While | FunctionDecl | VarDecl

### Simple Statement

Expression ";"

### If Statement

Keyword::If Parenthesized Bracketed
    | Keyword::If Parenthesized Bracketed Keyword::Else Bracketed
    | Keyword::If Parenthesized Bracketed $(Keyword::Else Keyword::If Parenthesized Bracketed)* $(Keyword::Else Bracketed)?

### For Statement

"for" Expression "in" Expression Bracketed

## While Statement

"while" Condition Bracketed

### FunctionDecl Statement

"$" Expression Parenthesized Bracketed

### VarDecl Statement

( "let" | "const" ) Expression

---

## Expression

Literal | Ident | Unary | Binary | Parenthesized | Bracketed
    | Braced | Call | Assign | Conditional

### Parenthesized Expression

"(" Expression ")"

### Bracketed Expression

"[" Expression "]"

### Braced Expression

"{" Expression "}"

### Call Expression

Expression Parenthesized

### Assign Expression

Expression "=" Expression

### Conditional Expression

Expression ( "<" | ">" | "<=" | ">=" | "==" | "!=" ) Expression

### Unary Expression

( "-" | "!" ) Expression

### Binary Expression

Expression Operator Expression

## Operator

"." | "++" | "--" | "+=" | "-=" | "\*="  "\*\*" | "=" | "/" | "/=" | "+" | "-" | "*"
