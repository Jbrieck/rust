//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `USE_KW`, or `STRUCT_DEF`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    SEMI,
    COMMA,
    L_PAREN,
    R_PAREN,
    L_CURLY,
    R_CURLY,
    L_BRACK,
    R_BRACK,
    L_ANGLE,
    R_ANGLE,
    AT,
    POUND,
    TILDE,
    QUESTION,
    DOLLAR,
    AMP,
    PIPE,
    PLUS,
    STAR,
    SLASH,
    CARET,
    PERCENT,
    UNDERSCORE,
    DOT,
    DOTDOT,
    DOTDOTDOT,
    DOTDOTEQ,
    COLON,
    COLONCOLON,
    EQ,
    EQEQ,
    FAT_ARROW,
    EXCL,
    NEQ,
    MINUS,
    THIN_ARROW,
    LTEQ,
    GTEQ,
    PLUSEQ,
    MINUSEQ,
    PIPEEQ,
    AMPEQ,
    CARETEQ,
    SLASHEQ,
    STAREQ,
    PERCENTEQ,
    AMPAMP,
    PIPEPIPE,
    SHL,
    SHR,
    SHLEQ,
    SHREQ,
    AS_KW,
    ASYNC_KW,
    AWAIT_KW,
    BOX_KW,
    BREAK_KW,
    CONST_KW,
    CONTINUE_KW,
    CRATE_KW,
    DYN_KW,
    ELSE_KW,
    ENUM_KW,
    EXTERN_KW,
    FALSE_KW,
    FN_KW,
    FOR_KW,
    IF_KW,
    IMPL_KW,
    IN_KW,
    LET_KW,
    LOOP_KW,
    MACRO_KW,
    MATCH_KW,
    MOD_KW,
    MOVE_KW,
    MUT_KW,
    PUB_KW,
    REF_KW,
    RETURN_KW,
    SELF_KW,
    STATIC_KW,
    STRUCT_KW,
    SUPER_KW,
    TRAIT_KW,
    TRUE_KW,
    TRY_KW,
    TYPE_KW,
    UNSAFE_KW,
    USE_KW,
    WHERE_KW,
    WHILE_KW,
    AUTO_KW,
    DEFAULT_KW,
    EXISTENTIAL_KW,
    UNION_KW,
    RAW_KW,
    INT_NUMBER,
    FLOAT_NUMBER,
    CHAR,
    BYTE,
    STRING,
    RAW_STRING,
    BYTE_STRING,
    RAW_BYTE_STRING,
    ERROR,
    IDENT,
    WHITESPACE,
    LIFETIME,
    COMMENT,
    SHEBANG,
    L_DOLLAR,
    R_DOLLAR,
    SOURCE_FILE,
    STRUCT_DEF,
    UNION_DEF,
    ENUM_DEF,
    FN_DEF,
    RET_TYPE,
    EXTERN_CRATE_ITEM,
    MODULE,
    USE_ITEM,
    STATIC_DEF,
    CONST_DEF,
    TRAIT_DEF,
    IMPL_DEF,
    TYPE_ALIAS_DEF,
    MACRO_CALL,
    TOKEN_TREE,
    MACRO_DEF,
    PAREN_TYPE,
    TUPLE_TYPE,
    NEVER_TYPE,
    PATH_TYPE,
    POINTER_TYPE,
    ARRAY_TYPE,
    SLICE_TYPE,
    REFERENCE_TYPE,
    PLACEHOLDER_TYPE,
    FN_POINTER_TYPE,
    FOR_TYPE,
    IMPL_TRAIT_TYPE,
    DYN_TRAIT_TYPE,
    OR_PAT,
    PAREN_PAT,
    REF_PAT,
    BOX_PAT,
    BIND_PAT,
    PLACEHOLDER_PAT,
    DOT_DOT_PAT,
    PATH_PAT,
    RECORD_PAT,
    RECORD_FIELD_PAT_LIST,
    RECORD_FIELD_PAT,
    TUPLE_STRUCT_PAT,
    TUPLE_PAT,
    SLICE_PAT,
    RANGE_PAT,
    LITERAL_PAT,
    MACRO_PAT,
    TUPLE_EXPR,
    ARRAY_EXPR,
    PAREN_EXPR,
    PATH_EXPR,
    LAMBDA_EXPR,
    IF_EXPR,
    WHILE_EXPR,
    CONDITION,
    LOOP_EXPR,
    FOR_EXPR,
    CONTINUE_EXPR,
    BREAK_EXPR,
    LABEL,
    BLOCK_EXPR,
    RETURN_EXPR,
    MATCH_EXPR,
    MATCH_ARM_LIST,
    MATCH_ARM,
    MATCH_GUARD,
    RECORD_LIT,
    RECORD_FIELD_LIST,
    RECORD_FIELD,
    TRY_BLOCK_EXPR,
    BOX_EXPR,
    CALL_EXPR,
    INDEX_EXPR,
    METHOD_CALL_EXPR,
    FIELD_EXPR,
    AWAIT_EXPR,
    TRY_EXPR,
    CAST_EXPR,
    REF_EXPR,
    PREFIX_EXPR,
    RANGE_EXPR,
    BIN_EXPR,
    BLOCK,
    EXTERN_BLOCK,
    EXTERN_ITEM_LIST,
    ENUM_VARIANT,
    RECORD_FIELD_DEF_LIST,
    RECORD_FIELD_DEF,
    TUPLE_FIELD_DEF_LIST,
    TUPLE_FIELD_DEF,
    ENUM_VARIANT_LIST,
    ITEM_LIST,
    ATTR,
    META_ITEM,
    USE_TREE,
    USE_TREE_LIST,
    PATH,
    PATH_SEGMENT,
    LITERAL,
    ALIAS,
    VISIBILITY,
    WHERE_CLAUSE,
    WHERE_PRED,
    ABI,
    NAME,
    NAME_REF,
    LET_STMT,
    EXPR_STMT,
    TYPE_PARAM_LIST,
    LIFETIME_PARAM,
    TYPE_PARAM,
    CONST_PARAM,
    TYPE_ARG_LIST,
    LIFETIME_ARG,
    TYPE_ARG,
    ASSOC_TYPE_ARG,
    CONST_ARG,
    PARAM_LIST,
    PARAM,
    SELF_PARAM,
    ARG_LIST,
    TYPE_BOUND,
    TYPE_BOUND_LIST,
    MACRO_ITEMS,
    MACRO_STMTS,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        match self {
            AS_KW | ASYNC_KW | AWAIT_KW | BOX_KW | BREAK_KW | CONST_KW | CONTINUE_KW | CRATE_KW
            | DYN_KW | ELSE_KW | ENUM_KW | EXTERN_KW | FALSE_KW | FN_KW | FOR_KW | IF_KW
            | IMPL_KW | IN_KW | LET_KW | LOOP_KW | MACRO_KW | MATCH_KW | MOD_KW | MOVE_KW
            | MUT_KW | PUB_KW | REF_KW | RETURN_KW | SELF_KW | STATIC_KW | STRUCT_KW | SUPER_KW
            | TRAIT_KW | TRUE_KW | TRY_KW | TYPE_KW | UNSAFE_KW | USE_KW | WHERE_KW | WHILE_KW
            | AUTO_KW | DEFAULT_KW | EXISTENTIAL_KW | UNION_KW | RAW_KW => true,
            _ => false,
        }
    }
    pub fn is_punct(self) -> bool {
        match self {
            SEMI | COMMA | L_PAREN | R_PAREN | L_CURLY | R_CURLY | L_BRACK | R_BRACK | L_ANGLE
            | R_ANGLE | AT | POUND | TILDE | QUESTION | DOLLAR | AMP | PIPE | PLUS | STAR
            | SLASH | CARET | PERCENT | UNDERSCORE | DOT | DOTDOT | DOTDOTDOT | DOTDOTEQ
            | COLON | COLONCOLON | EQ | EQEQ | FAT_ARROW | EXCL | NEQ | MINUS | THIN_ARROW
            | LTEQ | GTEQ | PLUSEQ | MINUSEQ | PIPEEQ | AMPEQ | CARETEQ | SLASHEQ | STAREQ
            | PERCENTEQ | AMPAMP | PIPEPIPE | SHL | SHR | SHLEQ | SHREQ => true,
            _ => false,
        }
    }
    pub fn is_literal(self) -> bool {
        match self {
            INT_NUMBER | FLOAT_NUMBER | CHAR | BYTE | STRING | RAW_STRING | BYTE_STRING
            | RAW_BYTE_STRING => true,
            _ => false,
        }
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "as" => AS_KW,
            "async" => ASYNC_KW,
            "await" => AWAIT_KW,
            "box" => BOX_KW,
            "break" => BREAK_KW,
            "const" => CONST_KW,
            "continue" => CONTINUE_KW,
            "crate" => CRATE_KW,
            "dyn" => DYN_KW,
            "else" => ELSE_KW,
            "enum" => ENUM_KW,
            "extern" => EXTERN_KW,
            "false" => FALSE_KW,
            "fn" => FN_KW,
            "for" => FOR_KW,
            "if" => IF_KW,
            "impl" => IMPL_KW,
            "in" => IN_KW,
            "let" => LET_KW,
            "loop" => LOOP_KW,
            "macro" => MACRO_KW,
            "match" => MATCH_KW,
            "mod" => MOD_KW,
            "move" => MOVE_KW,
            "mut" => MUT_KW,
            "pub" => PUB_KW,
            "ref" => REF_KW,
            "return" => RETURN_KW,
            "self" => SELF_KW,
            "static" => STATIC_KW,
            "struct" => STRUCT_KW,
            "super" => SUPER_KW,
            "trait" => TRAIT_KW,
            "true" => TRUE_KW,
            "try" => TRY_KW,
            "type" => TYPE_KW,
            "unsafe" => UNSAFE_KW,
            "use" => USE_KW,
            "where" => WHERE_KW,
            "while" => WHILE_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            ';' => SEMI,
            ',' => COMMA,
            '(' => L_PAREN,
            ')' => R_PAREN,
            '{' => L_CURLY,
            '}' => R_CURLY,
            '[' => L_BRACK,
            ']' => R_BRACK,
            '<' => L_ANGLE,
            '>' => R_ANGLE,
            '@' => AT,
            '#' => POUND,
            '~' => TILDE,
            '?' => QUESTION,
            '$' => DOLLAR,
            '&' => AMP,
            '|' => PIPE,
            '+' => PLUS,
            '*' => STAR,
            '/' => SLASH,
            '^' => CARET,
            '%' => PERCENT,
            '_' => UNDERSCORE,
            '.' => DOT,
            ':' => COLON,
            '=' => EQ,
            '!' => EXCL,
            '-' => MINUS,
            _ => return None,
        };
        Some(tok)
    }
}
#[macro_export]
macro_rules! T {
    ( ; ) => {
        $crate::SyntaxKind::SEMI
    };
    ( , ) => {
        $crate::SyntaxKind::COMMA
    };
    ( '(' ) => {
        $crate::SyntaxKind::L_PAREN
    };
    ( ')' ) => {
        $crate::SyntaxKind::R_PAREN
    };
    ( '{' ) => {
        $crate::SyntaxKind::L_CURLY
    };
    ( '}' ) => {
        $crate::SyntaxKind::R_CURLY
    };
    ( '[' ) => {
        $crate::SyntaxKind::L_BRACK
    };
    ( ']' ) => {
        $crate::SyntaxKind::R_BRACK
    };
    ( < ) => {
        $crate::SyntaxKind::L_ANGLE
    };
    ( > ) => {
        $crate::SyntaxKind::R_ANGLE
    };
    ( @ ) => {
        $crate::SyntaxKind::AT
    };
    ( # ) => {
        $crate::SyntaxKind::POUND
    };
    ( ~ ) => {
        $crate::SyntaxKind::TILDE
    };
    ( ? ) => {
        $crate::SyntaxKind::QUESTION
    };
    ( $ ) => {
        $crate::SyntaxKind::DOLLAR
    };
    ( & ) => {
        $crate::SyntaxKind::AMP
    };
    ( | ) => {
        $crate::SyntaxKind::PIPE
    };
    ( + ) => {
        $crate::SyntaxKind::PLUS
    };
    ( * ) => {
        $crate::SyntaxKind::STAR
    };
    ( / ) => {
        $crate::SyntaxKind::SLASH
    };
    ( ^ ) => {
        $crate::SyntaxKind::CARET
    };
    ( % ) => {
        $crate::SyntaxKind::PERCENT
    };
    ( _ ) => {
        $crate::SyntaxKind::UNDERSCORE
    };
    ( . ) => {
        $crate::SyntaxKind::DOT
    };
    ( .. ) => {
        $crate::SyntaxKind::DOTDOT
    };
    ( ... ) => {
        $crate::SyntaxKind::DOTDOTDOT
    };
    ( ..= ) => {
        $crate::SyntaxKind::DOTDOTEQ
    };
    ( : ) => {
        $crate::SyntaxKind::COLON
    };
    ( :: ) => {
        $crate::SyntaxKind::COLONCOLON
    };
    ( = ) => {
        $crate::SyntaxKind::EQ
    };
    ( == ) => {
        $crate::SyntaxKind::EQEQ
    };
    ( => ) => {
        $crate::SyntaxKind::FAT_ARROW
    };
    ( ! ) => {
        $crate::SyntaxKind::EXCL
    };
    ( != ) => {
        $crate::SyntaxKind::NEQ
    };
    ( - ) => {
        $crate::SyntaxKind::MINUS
    };
    ( -> ) => {
        $crate::SyntaxKind::THIN_ARROW
    };
    ( <= ) => {
        $crate::SyntaxKind::LTEQ
    };
    ( >= ) => {
        $crate::SyntaxKind::GTEQ
    };
    ( += ) => {
        $crate::SyntaxKind::PLUSEQ
    };
    ( -= ) => {
        $crate::SyntaxKind::MINUSEQ
    };
    ( |= ) => {
        $crate::SyntaxKind::PIPEEQ
    };
    ( &= ) => {
        $crate::SyntaxKind::AMPEQ
    };
    ( ^= ) => {
        $crate::SyntaxKind::CARETEQ
    };
    ( /= ) => {
        $crate::SyntaxKind::SLASHEQ
    };
    ( *= ) => {
        $crate::SyntaxKind::STAREQ
    };
    ( %= ) => {
        $crate::SyntaxKind::PERCENTEQ
    };
    ( && ) => {
        $crate::SyntaxKind::AMPAMP
    };
    ( || ) => {
        $crate::SyntaxKind::PIPEPIPE
    };
    ( << ) => {
        $crate::SyntaxKind::SHL
    };
    ( >> ) => {
        $crate::SyntaxKind::SHR
    };
    ( <<= ) => {
        $crate::SyntaxKind::SHLEQ
    };
    ( >>= ) => {
        $crate::SyntaxKind::SHREQ
    };
    ( as ) => {
        $crate::SyntaxKind::AS_KW
    };
    ( async ) => {
        $crate::SyntaxKind::ASYNC_KW
    };
    ( await ) => {
        $crate::SyntaxKind::AWAIT_KW
    };
    ( box ) => {
        $crate::SyntaxKind::BOX_KW
    };
    ( break ) => {
        $crate::SyntaxKind::BREAK_KW
    };
    ( const ) => {
        $crate::SyntaxKind::CONST_KW
    };
    ( continue ) => {
        $crate::SyntaxKind::CONTINUE_KW
    };
    ( crate ) => {
        $crate::SyntaxKind::CRATE_KW
    };
    ( dyn ) => {
        $crate::SyntaxKind::DYN_KW
    };
    ( else ) => {
        $crate::SyntaxKind::ELSE_KW
    };
    ( enum ) => {
        $crate::SyntaxKind::ENUM_KW
    };
    ( extern ) => {
        $crate::SyntaxKind::EXTERN_KW
    };
    ( false ) => {
        $crate::SyntaxKind::FALSE_KW
    };
    ( fn ) => {
        $crate::SyntaxKind::FN_KW
    };
    ( for ) => {
        $crate::SyntaxKind::FOR_KW
    };
    ( if ) => {
        $crate::SyntaxKind::IF_KW
    };
    ( impl ) => {
        $crate::SyntaxKind::IMPL_KW
    };
    ( in ) => {
        $crate::SyntaxKind::IN_KW
    };
    ( let ) => {
        $crate::SyntaxKind::LET_KW
    };
    ( loop ) => {
        $crate::SyntaxKind::LOOP_KW
    };
    ( macro ) => {
        $crate::SyntaxKind::MACRO_KW
    };
    ( match ) => {
        $crate::SyntaxKind::MATCH_KW
    };
    ( mod ) => {
        $crate::SyntaxKind::MOD_KW
    };
    ( move ) => {
        $crate::SyntaxKind::MOVE_KW
    };
    ( mut ) => {
        $crate::SyntaxKind::MUT_KW
    };
    ( pub ) => {
        $crate::SyntaxKind::PUB_KW
    };
    ( ref ) => {
        $crate::SyntaxKind::REF_KW
    };
    ( return ) => {
        $crate::SyntaxKind::RETURN_KW
    };
    ( self ) => {
        $crate::SyntaxKind::SELF_KW
    };
    ( static ) => {
        $crate::SyntaxKind::STATIC_KW
    };
    ( struct ) => {
        $crate::SyntaxKind::STRUCT_KW
    };
    ( super ) => {
        $crate::SyntaxKind::SUPER_KW
    };
    ( trait ) => {
        $crate::SyntaxKind::TRAIT_KW
    };
    ( true ) => {
        $crate::SyntaxKind::TRUE_KW
    };
    ( try ) => {
        $crate::SyntaxKind::TRY_KW
    };
    ( type ) => {
        $crate::SyntaxKind::TYPE_KW
    };
    ( unsafe ) => {
        $crate::SyntaxKind::UNSAFE_KW
    };
    ( use ) => {
        $crate::SyntaxKind::USE_KW
    };
    ( where ) => {
        $crate::SyntaxKind::WHERE_KW
    };
    ( while ) => {
        $crate::SyntaxKind::WHILE_KW
    };
    ( auto ) => {
        $crate::SyntaxKind::AUTO_KW
    };
    ( default ) => {
        $crate::SyntaxKind::DEFAULT_KW
    };
    ( existential ) => {
        $crate::SyntaxKind::EXISTENTIAL_KW
    };
    ( union ) => {
        $crate::SyntaxKind::UNION_KW
    };
    ( raw ) => {
        $crate::SyntaxKind::RAW_KW
    };
    ( lifetime ) => {
        $crate::SyntaxKind::LIFETIME
    };
    ( ident ) => {
        $crate::SyntaxKind::IDENT
    };
}
