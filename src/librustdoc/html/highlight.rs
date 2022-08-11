//! Basic syntax highlighting functionality.
//!
//! This module uses librustc_ast's lexer to provide token-based highlighting for
//! the HTML documentation generated by rustdoc.
//!
//! Use the `render_with_highlighting` to highlight some rust code.

use crate::clean::PrimitiveType;
use crate::html::escape::Escape;
use crate::html::render::Context;

use std::collections::VecDeque;
use std::fmt::{Display, Write};

use rustc_data_structures::fx::FxHashMap;
use rustc_lexer::{LiteralKind, TokenKind};
use rustc_span::edition::Edition;
use rustc_span::symbol::Symbol;
use rustc_span::{BytePos, Span, DUMMY_SP};

use super::format::{self, Buffer};
use super::render::LinkFromSrc;

/// This type is needed in case we want to render links on items to allow to go to their definition.
pub(crate) struct HrefContext<'a, 'b, 'c> {
    pub(crate) context: &'a Context<'b>,
    /// This span contains the current file we're going through.
    pub(crate) file_span: Span,
    /// This field is used to know "how far" from the top of the directory we are to link to either
    /// documentation pages or other source pages.
    pub(crate) root_path: &'c str,
}

/// Decorations are represented as a map from CSS class to vector of character ranges.
/// Each range will be wrapped in a span with that class.
#[derive(Default)]
pub(crate) struct DecorationInfo(pub(crate) FxHashMap<&'static str, Vec<(u32, u32)>>);

#[derive(Eq, PartialEq, Clone, Copy)]
pub(crate) enum Tooltip {
    Ignore,
    CompileFail,
    ShouldPanic,
    Edition(Edition),
    None,
}

/// Highlights `src` as an inline example, returning the HTML output.
pub(crate) fn render_example_with_highlighting(
    src: &str,
    out: &mut Buffer,
    tooltip: Tooltip,
    playground_button: Option<&str>,
) {
    let class = match tooltip {
        Tooltip::Ignore => " ignore",
        Tooltip::CompileFail => " compile_fail",
        Tooltip::ShouldPanic => " should_panic",
        Tooltip::Edition(_) => " edition",
        Tooltip::None => "",
    };

    if tooltip != Tooltip::None {
        write!(
            out,
            "<div class='information'><div class='tooltip{}'{}>ⓘ</div></div>",
            class,
            if let Tooltip::Edition(edition_info) = tooltip {
                format!(" data-edition=\"{}\"", edition_info)
            } else {
                String::new()
            },
        );
    }

    write_header(out, &format!("rust-example-rendered{}", class), None);
    write_code(out, src, None, None);
    write_footer(out, playground_button);
}

/// Highlights `src` as a macro, returning the HTML output.
pub(crate) fn render_macro_with_highlighting(src: &str, out: &mut Buffer) {
    write_header(out, "macro", None);
    write_code(out, src, None, None);
    write_footer(out, None);
}

/// Highlights `src` as a source code page, returning the HTML output.
pub(crate) fn render_source_with_highlighting(
    src: &str,
    out: &mut Buffer,
    line_numbers: Buffer,
    href_context: HrefContext<'_, '_, '_>,
    decoration_info: DecorationInfo,
) {
    write_header(out, "", Some(line_numbers));
    write_code(out, src, Some(href_context), Some(decoration_info));
    write_footer(out, None);
}

fn write_header(out: &mut Buffer, class: &str, extra_content: Option<Buffer>) {
    write!(out, "<div class=\"example-wrap\">");
    if let Some(extra) = extra_content {
        out.push_buffer(extra);
    }
    if class.is_empty() {
        write!(out, "<pre class=\"rust\">");
    } else {
        write!(out, "<pre class=\"rust {}\">", class);
    }
    write!(out, "<code>");
}

/// Convert the given `src` source code into HTML by adding classes for highlighting.
///
/// This code is used to render code blocks (in the documentation) as well as the source code pages.
///
/// Some explanations on the last arguments:
///
/// In case we are rendering a code block and not a source code file, `href_context` will be `None`.
/// To put it more simply: if `href_context` is `None`, the code won't try to generate links to an
/// item definition.
///
/// More explanations about spans and how we use them here are provided in the
fn write_code(
    out: &mut Buffer,
    src: &str,
    href_context: Option<HrefContext<'_, '_, '_>>,
    decoration_info: Option<DecorationInfo>,
) {
    // This replace allows to fix how the code source with DOS backline characters is displayed.
    let src = src.replace("\r\n", "\n");
    let mut closing_tags: Vec<&'static str> = Vec::new();
    Classifier::new(
        &src,
        href_context.as_ref().map(|c| c.file_span).unwrap_or(DUMMY_SP),
        decoration_info,
    )
    .highlight(&mut |highlight| {
        match highlight {
            Highlight::Token { text, class } => string(out, Escape(text), class, &href_context),
            Highlight::EnterSpan { class } => {
                closing_tags.push(enter_span(out, class, &href_context))
            }
            Highlight::ExitSpan => {
                exit_span(out, closing_tags.pop().expect("ExitSpan without EnterSpan"))
            }
        };
    });
}

fn write_footer(out: &mut Buffer, playground_button: Option<&str>) {
    writeln!(out, "</code></pre>{}</div>", playground_button.unwrap_or_default());
}

/// How a span of text is classified. Mostly corresponds to token kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Class {
    Comment,
    DocComment,
    Attribute,
    KeyWord,
    // Keywords that do pointer/reference stuff.
    RefKeyWord,
    Self_(Span),
    Op,
    Macro(Span),
    MacroNonTerminal,
    String,
    Number,
    Bool,
    Ident(Span),
    Lifetime,
    PreludeTy,
    PreludeVal,
    QuestionMark,
    Decoration(&'static str),
}

impl Class {
    /// Returns the css class expected by rustdoc for each `Class`.
    fn as_html(self) -> &'static str {
        match self {
            Class::Comment => "comment",
            Class::DocComment => "doccomment",
            Class::Attribute => "attribute",
            Class::KeyWord => "kw",
            Class::RefKeyWord => "kw-2",
            Class::Self_(_) => "self",
            Class::Op => "op",
            Class::Macro(_) => "macro",
            Class::MacroNonTerminal => "macro-nonterminal",
            Class::String => "string",
            Class::Number => "number",
            Class::Bool => "bool-val",
            Class::Ident(_) => "ident",
            Class::Lifetime => "lifetime",
            Class::PreludeTy => "prelude-ty",
            Class::PreludeVal => "prelude-val",
            Class::QuestionMark => "question-mark",
            Class::Decoration(kind) => kind,
        }
    }

    /// In case this is an item which can be converted into a link to a definition, it'll contain
    /// a "span" (a tuple representing `(lo, hi)` equivalent of `Span`).
    fn get_span(self) -> Option<Span> {
        match self {
            Self::Ident(sp) | Self::Self_(sp) | Self::Macro(sp) => Some(sp),
            Self::Comment
            | Self::DocComment
            | Self::Attribute
            | Self::KeyWord
            | Self::RefKeyWord
            | Self::Op
            | Self::MacroNonTerminal
            | Self::String
            | Self::Number
            | Self::Bool
            | Self::Lifetime
            | Self::PreludeTy
            | Self::PreludeVal
            | Self::QuestionMark
            | Self::Decoration(_) => None,
        }
    }
}

enum Highlight<'a> {
    Token { text: &'a str, class: Option<Class> },
    EnterSpan { class: Class },
    ExitSpan,
}

struct TokenIter<'a> {
    src: &'a str,
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = (TokenKind, &'a str);
    fn next(&mut self) -> Option<(TokenKind, &'a str)> {
        if self.src.is_empty() {
            return None;
        }
        let token = rustc_lexer::first_token(self.src);
        let (text, rest) = self.src.split_at(token.len as usize);
        self.src = rest;
        Some((token.kind, text))
    }
}

/// Classifies into identifier class; returns `None` if this is a non-keyword identifier.
fn get_real_ident_class(text: &str, allow_path_keywords: bool) -> Option<Class> {
    let ignore: &[&str] =
        if allow_path_keywords { &["self", "Self", "super", "crate"] } else { &["self", "Self"] };
    if ignore.iter().any(|k| *k == text) {
        return None;
    }
    Some(match text {
        "ref" | "mut" => Class::RefKeyWord,
        "false" | "true" => Class::Bool,
        _ if Symbol::intern(text).is_reserved(|| Edition::Edition2021) => Class::KeyWord,
        _ => return None,
    })
}

/// This iterator comes from the same idea than "Peekable" except that it allows to "peek" more than
/// just the next item by using `peek_next`. The `peek` method always returns the next item after
/// the current one whereas `peek_next` will return the next item after the last one peeked.
///
/// You can use both `peek` and `peek_next` at the same time without problem.
struct PeekIter<'a> {
    stored: VecDeque<(TokenKind, &'a str)>,
    /// This position is reinitialized when using `next`. It is used in `peek_next`.
    peek_pos: usize,
    iter: TokenIter<'a>,
}

impl<'a> PeekIter<'a> {
    fn new(iter: TokenIter<'a>) -> Self {
        Self { stored: VecDeque::new(), peek_pos: 0, iter }
    }
    /// Returns the next item after the current one. It doesn't interfer with `peek_next` output.
    fn peek(&mut self) -> Option<&(TokenKind, &'a str)> {
        if self.stored.is_empty() {
            if let Some(next) = self.iter.next() {
                self.stored.push_back(next);
            }
        }
        self.stored.front()
    }
    /// Returns the next item after the last one peeked. It doesn't interfer with `peek` output.
    fn peek_next(&mut self) -> Option<&(TokenKind, &'a str)> {
        self.peek_pos += 1;
        if self.peek_pos - 1 < self.stored.len() {
            self.stored.get(self.peek_pos - 1)
        } else if let Some(next) = self.iter.next() {
            self.stored.push_back(next);
            self.stored.back()
        } else {
            None
        }
    }
}

impl<'a> Iterator for PeekIter<'a> {
    type Item = (TokenKind, &'a str);
    fn next(&mut self) -> Option<Self::Item> {
        self.peek_pos = 0;
        if let Some(first) = self.stored.pop_front() { Some(first) } else { self.iter.next() }
    }
}

/// Custom spans inserted into the source. Eg --scrape-examples uses this to highlight function calls
struct Decorations {
    starts: Vec<(u32, &'static str)>,
    ends: Vec<u32>,
}

impl Decorations {
    fn new(info: DecorationInfo) -> Self {
        // Extract tuples (start, end, kind) into separate sequences of (start, kind) and (end).
        let (mut starts, mut ends): (Vec<_>, Vec<_>) = info
            .0
            .into_iter()
            .flat_map(|(kind, ranges)| ranges.into_iter().map(move |(lo, hi)| ((lo, kind), hi)))
            .unzip();

        // Sort the sequences in document order.
        starts.sort_by_key(|(lo, _)| *lo);
        ends.sort();

        Decorations { starts, ends }
    }
}

/// Processes program tokens, classifying strings of text by highlighting
/// category (`Class`).
struct Classifier<'a> {
    tokens: PeekIter<'a>,
    in_attribute: bool,
    in_macro: bool,
    in_macro_nonterminal: bool,
    byte_pos: u32,
    file_span: Span,
    src: &'a str,
    decorations: Option<Decorations>,
}

impl<'a> Classifier<'a> {
    /// Takes as argument the source code to HTML-ify, the rust edition to use and the source code
    /// file span which will be used later on by the `span_correspondance_map`.
    fn new(src: &str, file_span: Span, decoration_info: Option<DecorationInfo>) -> Classifier<'_> {
        let tokens = PeekIter::new(TokenIter { src });
        let decorations = decoration_info.map(Decorations::new);
        Classifier {
            tokens,
            in_attribute: false,
            in_macro: false,
            in_macro_nonterminal: false,
            byte_pos: 0,
            file_span,
            src,
            decorations,
        }
    }

    /// Convenient wrapper to create a [`Span`] from a position in the file.
    fn new_span(&self, lo: u32, text: &str) -> Span {
        let hi = lo + text.len() as u32;
        let file_lo = self.file_span.lo();
        self.file_span.with_lo(file_lo + BytePos(lo)).with_hi(file_lo + BytePos(hi))
    }

    /// Concatenate colons and idents as one when possible.
    fn get_full_ident_path(&mut self) -> Vec<(TokenKind, usize, usize)> {
        let start = self.byte_pos as usize;
        let mut pos = start;
        let mut has_ident = false;

        loop {
            let mut nb = 0;
            while let Some((TokenKind::Colon, _)) = self.tokens.peek() {
                self.tokens.next();
                nb += 1;
            }
            // Ident path can start with "::" but if we already have content in the ident path,
            // the "::" is mandatory.
            if has_ident && nb == 0 {
                return vec![(TokenKind::Ident, start, pos)];
            } else if nb != 0 && nb != 2 {
                if has_ident {
                    return vec![(TokenKind::Ident, start, pos), (TokenKind::Colon, pos, pos + nb)];
                } else {
                    return vec![(TokenKind::Colon, start, pos + nb)];
                }
            }

            if let Some((None, text)) = self.tokens.peek().map(|(token, text)| {
                if *token == TokenKind::Ident {
                    let class = get_real_ident_class(text, true);
                    (class, text)
                } else {
                    // Doesn't matter which Class we put in here...
                    (Some(Class::Comment), text)
                }
            }) {
                // We only "add" the colon if there is an ident behind.
                pos += text.len() + nb;
                has_ident = true;
                self.tokens.next();
            } else if nb > 0 && has_ident {
                return vec![(TokenKind::Ident, start, pos), (TokenKind::Colon, pos, pos + nb)];
            } else if nb > 0 {
                return vec![(TokenKind::Colon, start, start + nb)];
            } else if has_ident {
                return vec![(TokenKind::Ident, start, pos)];
            } else {
                return Vec::new();
            }
        }
    }

    /// Wraps the tokens iteration to ensure that the `byte_pos` is always correct.
    ///
    /// It returns the token's kind, the token as a string and its byte position in the source
    /// string.
    fn next(&mut self) -> Option<(TokenKind, &'a str, u32)> {
        if let Some((kind, text)) = self.tokens.next() {
            let before = self.byte_pos;
            self.byte_pos += text.len() as u32;
            Some((kind, text, before))
        } else {
            None
        }
    }

    /// Exhausts the `Classifier` writing the output into `sink`.
    ///
    /// The general structure for this method is to iterate over each token,
    /// possibly giving it an HTML span with a class specifying what flavor of
    /// token is used.
    fn highlight(mut self, sink: &mut dyn FnMut(Highlight<'a>)) {
        loop {
            if let Some(decs) = self.decorations.as_mut() {
                let byte_pos = self.byte_pos;
                let n_starts = decs.starts.iter().filter(|(i, _)| byte_pos >= *i).count();
                for (_, kind) in decs.starts.drain(0..n_starts) {
                    sink(Highlight::EnterSpan { class: Class::Decoration(kind) });
                }

                let n_ends = decs.ends.iter().filter(|i| byte_pos >= **i).count();
                for _ in decs.ends.drain(0..n_ends) {
                    sink(Highlight::ExitSpan);
                }
            }

            if self
                .tokens
                .peek()
                .map(|t| matches!(t.0, TokenKind::Colon | TokenKind::Ident))
                .unwrap_or(false)
            {
                let tokens = self.get_full_ident_path();
                for (token, start, end) in &tokens {
                    let text = &self.src[*start..*end];
                    self.advance(*token, text, sink, *start as u32);
                    self.byte_pos += text.len() as u32;
                }
                if !tokens.is_empty() {
                    continue;
                }
            }
            if let Some((token, text, before)) = self.next() {
                self.advance(token, text, sink, before);
            } else {
                break;
            }
        }
    }

    /// Single step of highlighting. This will classify `token`, but maybe also a couple of
    /// following ones as well.
    ///
    /// `before` is the position of the given token in the `source` string and is used as "lo" byte
    /// in case we want to try to generate a link for this token using the
    /// `span_correspondance_map`.
    fn advance(
        &mut self,
        token: TokenKind,
        text: &'a str,
        sink: &mut dyn FnMut(Highlight<'a>),
        before: u32,
    ) {
        let lookahead = self.peek();
        let no_highlight = |sink: &mut dyn FnMut(_)| sink(Highlight::Token { text, class: None });
        let class = match token {
            TokenKind::Whitespace => return no_highlight(sink),
            TokenKind::LineComment { doc_style } | TokenKind::BlockComment { doc_style, .. } => {
                if doc_style.is_some() {
                    Class::DocComment
                } else {
                    Class::Comment
                }
            }
            // Consider this as part of a macro invocation if there was a
            // leading identifier.
            TokenKind::Bang if self.in_macro => {
                self.in_macro = false;
                sink(Highlight::Token { text, class: None });
                sink(Highlight::ExitSpan);
                return;
            }

            // Assume that '&' or '*' is the reference or dereference operator
            // or a reference or pointer type. Unless, of course, it looks like
            // a logical and or a multiplication operator: `&&` or `* `.
            TokenKind::Star => match self.tokens.peek() {
                Some((TokenKind::Whitespace, _)) => Class::Op,
                Some((TokenKind::Ident, "mut")) => {
                    self.next();
                    sink(Highlight::Token { text: "*mut", class: Some(Class::RefKeyWord) });
                    return;
                }
                Some((TokenKind::Ident, "const")) => {
                    self.next();
                    sink(Highlight::Token { text: "*const", class: Some(Class::RefKeyWord) });
                    return;
                }
                _ => Class::RefKeyWord,
            },
            TokenKind::And => match self.tokens.peek() {
                Some((TokenKind::And, _)) => {
                    self.next();
                    sink(Highlight::Token { text: "&&", class: Some(Class::Op) });
                    return;
                }
                Some((TokenKind::Eq, _)) => {
                    self.next();
                    sink(Highlight::Token { text: "&=", class: Some(Class::Op) });
                    return;
                }
                Some((TokenKind::Whitespace, _)) => Class::Op,
                Some((TokenKind::Ident, "mut")) => {
                    self.next();
                    sink(Highlight::Token { text: "&mut", class: Some(Class::RefKeyWord) });
                    return;
                }
                _ => Class::RefKeyWord,
            },

            // These can either be operators, or arrows.
            TokenKind::Eq => match lookahead {
                Some(TokenKind::Eq) => {
                    self.next();
                    sink(Highlight::Token { text: "==", class: Some(Class::Op) });
                    return;
                }
                Some(TokenKind::Gt) => {
                    self.next();
                    sink(Highlight::Token { text: "=>", class: None });
                    return;
                }
                _ => Class::Op,
            },
            TokenKind::Minus if lookahead == Some(TokenKind::Gt) => {
                self.next();
                sink(Highlight::Token { text: "->", class: None });
                return;
            }

            // Other operators.
            TokenKind::Minus
            | TokenKind::Plus
            | TokenKind::Or
            | TokenKind::Slash
            | TokenKind::Caret
            | TokenKind::Percent
            | TokenKind::Bang
            | TokenKind::Lt
            | TokenKind::Gt => Class::Op,

            // Miscellaneous, no highlighting.
            TokenKind::Dot
            | TokenKind::Semi
            | TokenKind::Comma
            | TokenKind::OpenParen
            | TokenKind::CloseParen
            | TokenKind::OpenBrace
            | TokenKind::CloseBrace
            | TokenKind::OpenBracket
            | TokenKind::At
            | TokenKind::Tilde
            | TokenKind::Colon
            | TokenKind::Unknown => return no_highlight(sink),

            TokenKind::Question => Class::QuestionMark,

            TokenKind::Dollar => match lookahead {
                Some(TokenKind::Ident) => {
                    self.in_macro_nonterminal = true;
                    Class::MacroNonTerminal
                }
                _ => return no_highlight(sink),
            },

            // This might be the start of an attribute. We're going to want to
            // continue highlighting it as an attribute until the ending ']' is
            // seen, so skip out early. Down below we terminate the attribute
            // span when we see the ']'.
            TokenKind::Pound => {
                match lookahead {
                    // Case 1: #![inner_attribute]
                    Some(TokenKind::Bang) => {
                        self.next();
                        if let Some(TokenKind::OpenBracket) = self.peek() {
                            self.in_attribute = true;
                            sink(Highlight::EnterSpan { class: Class::Attribute });
                        }
                        sink(Highlight::Token { text: "#", class: None });
                        sink(Highlight::Token { text: "!", class: None });
                        return;
                    }
                    // Case 2: #[outer_attribute]
                    Some(TokenKind::OpenBracket) => {
                        self.in_attribute = true;
                        sink(Highlight::EnterSpan { class: Class::Attribute });
                    }
                    _ => (),
                }
                return no_highlight(sink);
            }
            TokenKind::CloseBracket => {
                if self.in_attribute {
                    self.in_attribute = false;
                    sink(Highlight::Token { text: "]", class: None });
                    sink(Highlight::ExitSpan);
                    return;
                }
                return no_highlight(sink);
            }
            TokenKind::Literal { kind, .. } => match kind {
                // Text literals.
                LiteralKind::Byte { .. }
                | LiteralKind::Char { .. }
                | LiteralKind::Str { .. }
                | LiteralKind::ByteStr { .. }
                | LiteralKind::RawStr { .. }
                | LiteralKind::RawByteStr { .. } => Class::String,
                // Number literals.
                LiteralKind::Float { .. } | LiteralKind::Int { .. } => Class::Number,
            },
            TokenKind::Ident | TokenKind::RawIdent if lookahead == Some(TokenKind::Bang) => {
                self.in_macro = true;
                sink(Highlight::EnterSpan { class: Class::Macro(self.new_span(before, text)) });
                sink(Highlight::Token { text, class: None });
                return;
            }
            TokenKind::Ident => match get_real_ident_class(text, false) {
                None => match text {
                    "Option" | "Result" => Class::PreludeTy,
                    "Some" | "None" | "Ok" | "Err" => Class::PreludeVal,
                    // "union" is a weak keyword and is only considered as a keyword when declaring
                    // a union type.
                    "union" if self.check_if_is_union_keyword() => Class::KeyWord,
                    _ if self.in_macro_nonterminal => {
                        self.in_macro_nonterminal = false;
                        Class::MacroNonTerminal
                    }
                    "self" | "Self" => Class::Self_(self.new_span(before, text)),
                    _ => Class::Ident(self.new_span(before, text)),
                },
                Some(c) => c,
            },
            TokenKind::RawIdent | TokenKind::UnknownPrefix | TokenKind::InvalidIdent => {
                Class::Ident(self.new_span(before, text))
            }
            TokenKind::Lifetime { .. } => Class::Lifetime,
        };
        // Anything that didn't return above is the simple case where we the
        // class just spans a single token, so we can use the `string` method.
        sink(Highlight::Token { text, class: Some(class) });
    }

    fn peek(&mut self) -> Option<TokenKind> {
        self.tokens.peek().map(|(token_kind, _text)| *token_kind)
    }

    fn check_if_is_union_keyword(&mut self) -> bool {
        while let Some(kind) = self.tokens.peek_next().map(|(token_kind, _text)| token_kind) {
            if *kind == TokenKind::Whitespace {
                continue;
            }
            return *kind == TokenKind::Ident;
        }
        false
    }
}

/// Called when we start processing a span of text that should be highlighted.
/// The `Class` argument specifies how it should be highlighted.
fn enter_span(
    out: &mut Buffer,
    klass: Class,
    href_context: &Option<HrefContext<'_, '_, '_>>,
) -> &'static str {
    string_without_closing_tag(out, "", Some(klass), href_context).expect(
        "internal error: enter_span was called with Some(klass) but did not return a \
            closing HTML tag",
    )
}

/// Called at the end of a span of highlighted text.
fn exit_span(out: &mut Buffer, closing_tag: &str) {
    out.write_str(closing_tag);
}

/// Called for a span of text. If the text should be highlighted differently
/// from the surrounding text, then the `Class` argument will be a value other
/// than `None`.
///
/// The following sequences of callbacks are equivalent:
/// ```plain
///     enter_span(Foo), string("text", None), exit_span()
///     string("text", Foo)
/// ```
///
/// The latter can be thought of as a shorthand for the former, which is more
/// flexible.
///
/// Note that if `context` is not `None` and that the given `klass` contains a `Span`, the function
/// will then try to find this `span` in the `span_correspondance_map`. If found, it'll then
/// generate a link for this element (which corresponds to where its definition is located).
fn string<T: Display>(
    out: &mut Buffer,
    text: T,
    klass: Option<Class>,
    href_context: &Option<HrefContext<'_, '_, '_>>,
) {
    if let Some(closing_tag) = string_without_closing_tag(out, text, klass, href_context) {
        out.write_str(closing_tag);
    }
}

/// This function writes `text` into `out` with some modifications depending on `klass`:
///
/// * If `klass` is `None`, `text` is written into `out` with no modification.
/// * If `klass` is `Some` but `klass.get_span()` is `None`, it writes the text wrapped in a
///   `<span>` with the provided `klass`.
/// * If `klass` is `Some` and has a [`rustc_span::Span`], it then tries to generate a link (`<a>`
///   element) by retrieving the link information from the `span_correspondance_map` that was filled
///   in `span_map.rs::collect_spans_and_sources`. If it cannot retrieve the information, then it's
///   the same as the second point (`klass` is `Some` but doesn't have a [`rustc_span::Span`]).
fn string_without_closing_tag<T: Display>(
    out: &mut Buffer,
    text: T,
    klass: Option<Class>,
    href_context: &Option<HrefContext<'_, '_, '_>>,
) -> Option<&'static str> {
    let Some(klass) = klass
    else {
        write!(out, "{}", text);
        return None;
    };
    let Some(def_span) = klass.get_span()
    else {
        write!(out, "<span class=\"{}\">{}", klass.as_html(), text);
        return Some("</span>");
    };

    let mut text_s = text.to_string();
    if text_s.contains("::") {
        text_s = text_s.split("::").intersperse("::").fold(String::new(), |mut path, t| {
            match t {
                "self" | "Self" => write!(
                    &mut path,
                    "<span class=\"{}\">{}</span>",
                    Class::Self_(DUMMY_SP).as_html(),
                    t
                ),
                "crate" | "super" => {
                    write!(&mut path, "<span class=\"{}\">{}</span>", Class::KeyWord.as_html(), t)
                }
                t => write!(&mut path, "{}", t),
            }
            .expect("Failed to build source HTML path");
            path
        });
    }
    if let Some(href_context) = href_context {
        if let Some(href) =
            href_context.context.shared.span_correspondance_map.get(&def_span).and_then(|href| {
                let context = href_context.context;
                // FIXME: later on, it'd be nice to provide two links (if possible) for all items:
                // one to the documentation page and one to the source definition.
                // FIXME: currently, external items only generate a link to their documentation,
                // a link to their definition can be generated using this:
                // https://github.com/rust-lang/rust/blob/60f1a2fc4b535ead9c85ce085fdce49b1b097531/src/librustdoc/html/render/context.rs#L315-L338
                match href {
                    LinkFromSrc::Local(span) => context
                        .href_from_span(*span, true)
                        .map(|s| format!("{}{}", href_context.root_path, s)),
                    LinkFromSrc::External(def_id) => {
                        format::href_with_root_path(*def_id, context, Some(href_context.root_path))
                            .ok()
                            .map(|(url, _, _)| url)
                    }
                    LinkFromSrc::Primitive(prim) => format::href_with_root_path(
                        PrimitiveType::primitive_locations(context.tcx())[prim],
                        context,
                        Some(href_context.root_path),
                    )
                    .ok()
                    .map(|(url, _, _)| url),
                }
            })
        {
            write!(out, "<a class=\"{}\" href=\"{}\">{}", klass.as_html(), href, text_s);
            return Some("</a>");
        }
    }
    write!(out, "<span class=\"{}\">{}", klass.as_html(), text_s);
    Some("</span>")
}

#[cfg(test)]
mod tests;
