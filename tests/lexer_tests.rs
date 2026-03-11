// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

mod common;

use halo::lexer::{Lexer, Token, TokenKind};

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Collect every token from `source` (including the terminal `Eof`).
fn tokenize(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source.to_string());
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        let done = tok.kind == TokenKind::Eof;
        tokens.push(tok);
        if done {
            break;
        }
    }
    tokens
}

/// Collect only the token *kinds*, discarding lexemes and positions.
fn kinds(source: &str) -> Vec<TokenKind> {
    tokenize(source).into_iter().map(|t| t.kind).collect()
}

/// Collect only the token *lexemes* (the raw matched text).
fn lexemes(source: &str) -> Vec<String> {
    tokenize(source).into_iter().map(|t| t.lexeme).collect()
}

/// Return the single non-Eof token produced from `source`.
///
/// Panics if the source does not produce exactly one token before Eof.
fn single(source: &str) -> Token {
    let mut toks = tokenize(source);
    // Last token is always Eof; remove it.
    let eof = toks.pop().unwrap();
    assert_eq!(eof.kind, TokenKind::Eof);
    assert_eq!(
        toks.len(),
        1,
        "expected exactly 1 token, got {}: {:?}",
        toks.len(),
        toks.iter().map(|t| &t.kind).collect::<Vec<_>>()
    );
    toks.remove(0)
}

// ── End-of-file ───────────────────────────────────────────────────────────────

#[test]
fn test_empty_source_produces_only_eof() {
    let toks = tokenize("");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].kind, TokenKind::Eof);
}

#[test]
fn test_whitespace_only_produces_only_eof() {
    let toks = tokenize("   \t  ");
    assert_eq!(toks.len(), 1);
    assert_eq!(toks[0].kind, TokenKind::Eof);
}

#[test]
fn test_eof_lexeme_is_empty() {
    let eof = tokenize("").remove(0);
    assert_eq!(eof.lexeme, "");
}

// ── Keywords ──────────────────────────────────────────────────────────────────

#[test]
fn test_keyword_if() {
    assert_eq!(single("if").kind, TokenKind::If);
}

#[test]
fn test_keyword_else() {
    assert_eq!(single("else").kind, TokenKind::Else);
}

#[test]
fn test_keyword_while() {
    assert_eq!(single("while").kind, TokenKind::While);
}

#[test]
fn test_keyword_return() {
    assert_eq!(single("return").kind, TokenKind::Return);
}

#[test]
fn test_keyword_true() {
    assert_eq!(single("true").kind, TokenKind::True);
}

#[test]
fn test_keyword_false() {
    assert_eq!(single("false").kind, TokenKind::False);
}

#[test]
fn test_keyword_break() {
    assert_eq!(single("break").kind, TokenKind::Break);
}

#[test]
fn test_keyword_continue() {
    assert_eq!(single("continue").kind, TokenKind::Continue);
}

#[test]
fn test_keyword_and_word() {
    assert_eq!(single("and").kind, TokenKind::And);
}

#[test]
fn test_keyword_or_word() {
    assert_eq!(single("or").kind, TokenKind::Or);
}

#[test]
fn test_keyword_not_word() {
    assert_eq!(single("not").kind, TokenKind::Not);
}

#[test]
fn test_keyword_lexeme_preserved() {
    assert_eq!(single("while").lexeme, "while");
    assert_eq!(single("return").lexeme, "return");
    assert_eq!(single("true").lexeme, "true");
}

// ── Identifiers ───────────────────────────────────────────────────────────────

#[test]
fn test_simple_identifier() {
    let tok = single("foo");
    assert_eq!(tok.kind, TokenKind::Identifier);
    assert_eq!(tok.lexeme, "foo");
}

#[test]
fn test_identifier_with_underscore_prefix() {
    let tok = single("_bar");
    assert_eq!(tok.kind, TokenKind::Identifier);
    assert_eq!(tok.lexeme, "_bar");
}

#[test]
fn test_identifier_with_digits() {
    let tok = single("my_var2");
    assert_eq!(tok.kind, TokenKind::Identifier);
    assert_eq!(tok.lexeme, "my_var2");
}

#[test]
fn test_identifier_all_underscores() {
    let tok = single("___");
    assert_eq!(tok.kind, TokenKind::Identifier);
}

#[test]
fn test_identifier_mixed_case() {
    let tok = single("CamelCase");
    assert_eq!(tok.kind, TokenKind::Identifier);
    assert_eq!(tok.lexeme, "CamelCase");
}

#[test]
fn test_keyword_prefix_does_not_consume_identifier() {
    // "ifx" is an identifier, not the keyword `if` followed by `x`.
    let tok = single("ifx");
    assert_eq!(tok.kind, TokenKind::Identifier);
    assert_eq!(tok.lexeme, "ifx");
}

#[test]
fn test_true_prefix_identifier() {
    // "trueness" is an identifier, not `true` + `ness`.
    let tok = single("trueness");
    assert_eq!(tok.kind, TokenKind::Identifier);
}

#[test]
fn test_while_prefix_identifier() {
    let tok = single("whileloop");
    assert_eq!(tok.kind, TokenKind::Identifier);
}

// ── Integer literals ──────────────────────────────────────────────────────────

#[test]
fn test_single_digit() {
    let tok = single("7");
    assert_eq!(tok.kind, TokenKind::Number);
    assert_eq!(tok.lexeme, "7");
}

#[test]
fn test_multi_digit_integer() {
    let tok = single("12345");
    assert_eq!(tok.kind, TokenKind::Number);
    assert_eq!(tok.lexeme, "12345");
}

#[test]
fn test_zero_literal() {
    let tok = single("0");
    assert_eq!(tok.kind, TokenKind::Number);
    assert_eq!(tok.lexeme, "0");
}

#[test]
fn test_large_integer_literal() {
    let tok = single("9223372036854775807");
    assert_eq!(tok.kind, TokenKind::Number);
    assert_eq!(tok.lexeme, "9223372036854775807");
}

// ── Floating-point literals ───────────────────────────────────────────────────

#[test]
fn test_float_literal_basic() {
    let tok = single("3.14");
    assert_eq!(tok.kind, TokenKind::Number);
    assert_eq!(tok.lexeme, "3.14");
}

#[test]
fn test_float_literal_zero_dot_something() {
    let tok = single("0.5");
    assert_eq!(tok.kind, TokenKind::Number);
    assert_eq!(tok.lexeme, "0.5");
}

#[test]
fn test_float_literal_trailing_digits() {
    let tok = single("1.000");
    assert_eq!(tok.kind, TokenKind::Number);
    assert_eq!(tok.lexeme, "1.000");
}

#[test]
fn test_integer_dot_integer_produces_number_dot_number() {
    // "1.2.3" → Number("1.2"), Dot, Number("3")
    let ks = kinds("1.2.3");
    assert_eq!(
        ks,
        vec![
            TokenKind::Number,
            TokenKind::Dot,
            TokenKind::Number,
            TokenKind::Eof
        ]
    );
}

#[test]
fn test_integer_followed_by_dot_without_digit_is_integer_then_dot() {
    // "42." — the dot has no following digit, so lexer emits Number + Dot.
    let ks = kinds("42.");
    assert_eq!(ks, vec![TokenKind::Number, TokenKind::Dot, TokenKind::Eof]);
    let ls = lexemes("42.");
    assert_eq!(ls[0], "42");
}

// ── String literals ───────────────────────────────────────────────────────────

#[test]
fn test_simple_string_literal() {
    let tok = single(r#""hello""#);
    assert_eq!(tok.kind, TokenKind::StringLit);
    assert_eq!(tok.lexeme, "hello");
}

#[test]
fn test_empty_string_literal() {
    let tok = single(r#""""#);
    assert_eq!(tok.kind, TokenKind::StringLit);
    assert_eq!(tok.lexeme, "");
}

#[test]
fn test_string_with_spaces() {
    let tok = single(r#""hello world""#);
    assert_eq!(tok.kind, TokenKind::StringLit);
    assert_eq!(tok.lexeme, "hello world");
}

#[test]
fn test_string_with_digits_and_symbols() {
    let tok = single(r#""abc123!@#""#);
    assert_eq!(tok.kind, TokenKind::StringLit);
    assert_eq!(tok.lexeme, "abc123!@#");
}

// ── String escape sequences ───────────────────────────────────────────────────

#[test]
fn test_string_escape_newline() {
    let tok = single(r#""\n""#);
    assert_eq!(tok.kind, TokenKind::StringLit);
    assert_eq!(tok.lexeme, "\n");
}

#[test]
fn test_string_escape_tab() {
    let tok = single(r#""\t""#);
    assert_eq!(tok.lexeme, "\t");
}

#[test]
fn test_string_escape_carriage_return() {
    let tok = single(r#""\r""#);
    assert_eq!(tok.lexeme, "\r");
}

#[test]
fn test_string_escape_backslash() {
    let tok = single(r#""\\""#);
    assert_eq!(tok.lexeme, "\\");
}

#[test]
fn test_string_escape_double_quote() {
    let tok = single(r#""\"""#);
    assert_eq!(tok.lexeme, "\"");
}

#[test]
fn test_string_escape_sequences_combined() {
    let tok = single(r#""a\tb\nc""#);
    assert_eq!(tok.lexeme, "a\tb\nc");
}

#[test]
fn test_string_unknown_escape_passes_through_both_chars() {
    // "\z" is not a recognised escape; lexer emits backslash + 'z'.
    let tok = single(r#""\z""#);
    assert_eq!(tok.lexeme, "\\z");
}

#[test]
fn test_unterminated_string_produces_unknown_token() {
    // Source ends without a closing quote.
    let tok = single(r#""unterminated"#);
    assert_eq!(tok.kind, TokenKind::Unknown);
}

// ── Arithmetic operators ──────────────────────────────────────────────────────

#[test]
fn test_plus_operator() {
    let tok = single("+");
    assert_eq!(tok.kind, TokenKind::Plus);
    assert_eq!(tok.lexeme, "+");
}

#[test]
fn test_minus_operator() {
    let tok = single("-");
    assert_eq!(tok.kind, TokenKind::Minus);
}

#[test]
fn test_star_operator() {
    let tok = single("*");
    assert_eq!(tok.kind, TokenKind::Star);
}

#[test]
fn test_slash_operator() {
    let tok = single("/");
    assert_eq!(tok.kind, TokenKind::Slash);
}

#[test]
fn test_modulo_operator() {
    let tok = single("%");
    assert_eq!(tok.kind, TokenKind::Modulo);
}

// ── Assignment ────────────────────────────────────────────────────────────────

#[test]
fn test_single_equals_is_assign() {
    let tok = single("=");
    assert_eq!(tok.kind, TokenKind::Assign);
}

// ── Comparison operators ──────────────────────────────────────────────────────

#[test]
fn test_double_equals_is_equal() {
    let tok = single("==");
    assert_eq!(tok.kind, TokenKind::Equal);
    assert_eq!(tok.lexeme, "==");
}

#[test]
fn test_not_equal_operator() {
    let tok = single("!=");
    assert_eq!(tok.kind, TokenKind::NotEqual);
    assert_eq!(tok.lexeme, "!=");
}

#[test]
fn test_less_than_operator() {
    let tok = single("<");
    assert_eq!(tok.kind, TokenKind::Less);
}

#[test]
fn test_greater_than_operator() {
    let tok = single(">");
    assert_eq!(tok.kind, TokenKind::Greater);
}

#[test]
fn test_less_equal_operator() {
    let tok = single("<=");
    assert_eq!(tok.kind, TokenKind::LessEqual);
    assert_eq!(tok.lexeme, "<=");
}

#[test]
fn test_greater_equal_operator() {
    let tok = single(">=");
    assert_eq!(tok.kind, TokenKind::GreaterEqual);
    assert_eq!(tok.lexeme, ">=");
}

#[test]
fn test_single_less_is_not_less_equal() {
    // "<" alone must not consume the next character.
    let ks = kinds("< =");
    assert_eq!(ks[0], TokenKind::Less);
    assert_eq!(ks[1], TokenKind::Assign);
}

#[test]
fn test_single_greater_is_not_greater_equal() {
    let ks = kinds("> =");
    assert_eq!(ks[0], TokenKind::Greater);
    assert_eq!(ks[1], TokenKind::Assign);
}

// ── Logical operators ─────────────────────────────────────────────────────────

#[test]
fn test_logical_and_symbol() {
    let tok = single("&&");
    assert_eq!(tok.kind, TokenKind::And);
    assert_eq!(tok.lexeme, "&&");
}

#[test]
fn test_logical_or_symbol() {
    let tok = single("||");
    assert_eq!(tok.kind, TokenKind::Or);
    assert_eq!(tok.lexeme, "||");
}

#[test]
fn test_logical_not_symbol() {
    let tok = single("!");
    assert_eq!(tok.kind, TokenKind::Not);
    assert_eq!(tok.lexeme, "!");
}

#[test]
fn test_single_ampersand_is_unknown() {
    let tok = single("&");
    assert_eq!(tok.kind, TokenKind::Unknown);
}

#[test]
fn test_single_pipe_is_unknown() {
    let tok = single("|");
    assert_eq!(tok.kind, TokenKind::Unknown);
}

// ── Delimiters ────────────────────────────────────────────────────────────────

#[test]
fn test_left_paren() {
    assert_eq!(single("(").kind, TokenKind::LeftParen);
}

#[test]
fn test_right_paren() {
    assert_eq!(single(")").kind, TokenKind::RightParen);
}

#[test]
fn test_left_brace() {
    assert_eq!(single("{").kind, TokenKind::LeftBrace);
}

#[test]
fn test_right_brace() {
    assert_eq!(single("}").kind, TokenKind::RightBrace);
}

#[test]
fn test_left_bracket() {
    assert_eq!(single("[").kind, TokenKind::LeftBracket);
}

#[test]
fn test_right_bracket() {
    assert_eq!(single("]").kind, TokenKind::RightBracket);
}

#[test]
fn test_colon() {
    assert_eq!(single(":").kind, TokenKind::Colon);
}

#[test]
fn test_comma() {
    assert_eq!(single(",").kind, TokenKind::Comma);
}

#[test]
fn test_dot_alone() {
    assert_eq!(single(".").kind, TokenKind::Dot);
}

// ── Newline ───────────────────────────────────────────────────────────────────

#[test]
fn test_newline_token_produced() {
    let ks = kinds("a\nb");
    assert!(
        ks.contains(&TokenKind::Newline),
        "expected a Newline token in {ks:?}"
    );
}

#[test]
fn test_newline_lexeme() {
    let toks = tokenize("a\nb");
    let newline = toks.iter().find(|t| t.kind == TokenKind::Newline).unwrap();
    assert_eq!(newline.lexeme, "\n");
}

#[test]
fn test_multiple_newlines_produce_multiple_tokens() {
    let toks = tokenize("\n\n\n");
    let count = toks.iter().filter(|t| t.kind == TokenKind::Newline).count();
    assert_eq!(count, 3);
}

// ── Comments ──────────────────────────────────────────────────────────────────

#[test]
fn test_line_comment_produces_no_token() {
    // The comment itself must not produce a Comment token.
    let ks = kinds("// this is a comment");
    assert!(!ks.contains(&TokenKind::Comment));
}

#[test]
fn test_line_comment_does_not_consume_newline() {
    // The newline after the comment must still appear as a token.
    let ks = kinds("// comment\nx");
    assert!(ks.contains(&TokenKind::Newline));
    assert!(ks.contains(&TokenKind::Identifier));
}

#[test]
fn test_inline_comment_leaves_preceding_token_intact() {
    let ks = kinds("x = 5 // comment");
    assert_eq!(ks[0], TokenKind::Identifier);
    assert_eq!(ks[1], TokenKind::Assign);
    assert_eq!(ks[2], TokenKind::Number);
    assert_eq!(ks[3], TokenKind::Eof);
}

#[test]
fn test_empty_comment_produces_no_extra_token() {
    let ks = kinds("x = 1 //");
    // Should be: Identifier, Assign, Number, Eof (no Comment token).
    assert_eq!(ks.len(), 4);
}

#[test]
fn test_comment_with_url_inside() {
    let ks = kinds("x = 1 // https://example.com");
    assert_eq!(ks[0], TokenKind::Identifier);
    assert_eq!(ks[1], TokenKind::Assign);
    assert_eq!(ks[2], TokenKind::Number);
    assert_eq!(ks[3], TokenKind::Eof);
}

#[test]
fn test_comment_with_symbols_inside() {
    let ks = kinds("y = 2 // @#$%^&*()");
    assert_eq!(ks[0], TokenKind::Identifier);
    assert_eq!(ks[3], TokenKind::Eof);
}

#[test]
fn test_code_after_comment_line_is_tokenised() {
    let ks = kinds("// skip\nx = 1");
    // Newline, Identifier, Assign, Number, Eof
    assert!(ks.contains(&TokenKind::Identifier));
    assert!(ks.contains(&TokenKind::Number));
}

#[test]
fn test_consecutive_comment_lines_are_consumed() {
    let ks = kinds("// one\n// two\n// three\nresult = 42");
    assert_eq!(ks[0], TokenKind::Newline); // after first comment
    assert!(ks.contains(&TokenKind::Identifier));
    assert!(ks.contains(&TokenKind::Number));
}

#[test]
fn test_single_slash_is_division_not_comment() {
    let ks = kinds("10 / 2");
    assert_eq!(ks[0], TokenKind::Number);
    assert_eq!(ks[1], TokenKind::Slash);
    assert_eq!(ks[2], TokenKind::Number);
}

#[test]
fn test_division_followed_by_comment() {
    // "10 / 2 // note" → Number, Slash, Number, Eof (comment consumed).
    let ks = kinds("10 / 2 // note");
    assert_eq!(ks[0], TokenKind::Number);
    assert_eq!(ks[1], TokenKind::Slash);
    assert_eq!(ks[2], TokenKind::Number);
    assert_eq!(ks[3], TokenKind::Eof);
}

// ── Source positions ──────────────────────────────────────────────────────────

#[test]
fn test_first_token_starts_at_line_1_column_1() {
    let tok = single("x");
    assert_eq!(tok.position.line, 1);
    assert_eq!(tok.position.column, 1);
}

#[test]
fn test_second_token_column_advances() {
    let toks = tokenize("a b");
    let b_tok = toks.iter().find(|t| t.lexeme == "b").unwrap();
    // 'a' is at column 1, ' ' is consumed, 'b' is at column 3.
    assert_eq!(b_tok.position.column, 3);
    assert_eq!(b_tok.position.line, 1);
}

#[test]
fn test_token_on_second_line() {
    let toks = tokenize("a\nb");
    let b_tok = toks.iter().find(|t| t.lexeme == "b").unwrap();
    assert_eq!(b_tok.position.line, 2);
    assert_eq!(b_tok.position.column, 1);
}

#[test]
fn test_token_on_third_line() {
    let toks = tokenize("a\nb\nc");
    let c_tok = toks.iter().find(|t| t.lexeme == "c").unwrap();
    assert_eq!(c_tok.position.line, 3);
}

#[test]
fn test_newline_token_line_number() {
    let toks = tokenize("a\n");
    let nl = toks.iter().find(|t| t.kind == TokenKind::Newline).unwrap();
    assert_eq!(nl.position.line, 1);
}

// ── Multi-token sequences ─────────────────────────────────────────────────────

#[test]
fn test_simple_assignment_tokens() {
    let ks = kinds("x = 42");
    assert_eq!(
        ks,
        vec![
            TokenKind::Identifier,
            TokenKind::Assign,
            TokenKind::Number,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_function_call_tokens() {
    let ks = kinds("foo(1, 2)");
    assert_eq!(
        ks,
        vec![
            TokenKind::Identifier,
            TokenKind::LeftParen,
            TokenKind::Number,
            TokenKind::Comma,
            TokenKind::Number,
            TokenKind::RightParen,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_if_else_block_tokens() {
    let ks = kinds("if x { a } else { b }");
    assert_eq!(ks[0], TokenKind::If);
    assert_eq!(ks[1], TokenKind::Identifier); // x
    assert_eq!(ks[2], TokenKind::LeftBrace);
    assert!(ks.contains(&TokenKind::Else));
}

#[test]
fn test_while_loop_tokens() {
    let ks = kinds("while i < 10 { i = i + 1 }");
    assert_eq!(ks[0], TokenKind::While);
    assert!(ks.contains(&TokenKind::Less));
    assert!(ks.contains(&TokenKind::LeftBrace));
    assert!(ks.contains(&TokenKind::Plus));
    assert!(ks.contains(&TokenKind::RightBrace));
}

#[test]
fn test_return_statement_tokens() {
    let ks = kinds("return n + 1");
    assert_eq!(ks[0], TokenKind::Return);
    assert_eq!(ks[1], TokenKind::Identifier);
    assert_eq!(ks[2], TokenKind::Plus);
    assert_eq!(ks[3], TokenKind::Number);
}

#[test]
fn test_logical_expression_tokens() {
    let ks = kinds("a && b || !c");
    assert_eq!(ks[0], TokenKind::Identifier); // a
    assert_eq!(ks[1], TokenKind::And);
    assert_eq!(ks[2], TokenKind::Identifier); // b
    assert_eq!(ks[3], TokenKind::Or);
    assert_eq!(ks[4], TokenKind::Not);
    assert_eq!(ks[5], TokenKind::Identifier); // c
}

#[test]
fn test_comparison_chain_tokens() {
    let ks = kinds("a == b != c");
    assert_eq!(ks[1], TokenKind::Equal);
    assert_eq!(ks[3], TokenKind::NotEqual);
}

#[test]
fn test_function_definition_tokens() {
    let ks = kinds("add(a, b) { return a + b }");
    assert_eq!(ks[0], TokenKind::Identifier); // add
    assert_eq!(ks[1], TokenKind::LeftParen);
    assert_eq!(ks[2], TokenKind::Identifier); // a
    assert_eq!(ks[3], TokenKind::Comma);
    assert_eq!(ks[4], TokenKind::Identifier); // b
    assert_eq!(ks[5], TokenKind::RightParen);
    assert_eq!(ks[6], TokenKind::LeftBrace);
    assert!(ks.contains(&TokenKind::Return));
    assert!(ks.contains(&TokenKind::RightBrace));
}

// ── Lexeme accuracy ───────────────────────────────────────────────────────────

#[test]
fn test_lexeme_of_identifier_is_exact_text() {
    let tok = single("myVariable");
    assert_eq!(tok.lexeme, "myVariable");
}

#[test]
fn test_lexeme_of_integer_is_exact_digits() {
    let tok = single("12345");
    assert_eq!(tok.lexeme, "12345");
}

#[test]
fn test_lexeme_of_float_includes_dot() {
    let tok = single("9.99");
    assert_eq!(tok.lexeme, "9.99");
}

#[test]
fn test_lexeme_of_operator_is_exact() {
    assert_eq!(single("<=").lexeme, "<=");
    assert_eq!(single(">=").lexeme, ">=");
    assert_eq!(single("==").lexeme, "==");
    assert_eq!(single("!=").lexeme, "!=");
    assert_eq!(single("&&").lexeme, "&&");
    assert_eq!(single("||").lexeme, "||");
}

// ── Unknown characters ────────────────────────────────────────────────────────

#[test]
fn test_dollar_sign_is_unknown() {
    let tok = single("$");
    assert_eq!(tok.kind, TokenKind::Unknown);
    assert_eq!(tok.lexeme, "$");
}

#[test]
fn test_at_sign_is_unknown() {
    let tok = single("@");
    assert_eq!(tok.kind, TokenKind::Unknown);
}

#[test]
fn test_hash_is_unknown() {
    let tok = single("#");
    assert_eq!(tok.kind, TokenKind::Unknown);
}

#[test]
fn test_unknown_does_not_stop_tokenisation() {
    // After an unknown token the lexer must continue with the rest.
    let ks = kinds("x $ y");
    assert_eq!(ks[0], TokenKind::Identifier); // x
    assert_eq!(ks[1], TokenKind::Unknown); // $
    assert_eq!(ks[2], TokenKind::Identifier); // y
    assert_eq!(ks[3], TokenKind::Eof);
}

// ── Whitespace handling ───────────────────────────────────────────────────────

#[test]
fn test_spaces_between_tokens_consumed_silently() {
    let ks = kinds("a   +   b");
    assert_eq!(
        ks,
        vec![
            TokenKind::Identifier,
            TokenKind::Plus,
            TokenKind::Identifier,
            TokenKind::Eof,
        ]
    );
}

#[test]
fn test_tabs_between_tokens_consumed_silently() {
    let ks = kinds("a\t+\tb");
    assert_eq!(ks[0], TokenKind::Identifier);
    assert_eq!(ks[1], TokenKind::Plus);
    assert_eq!(ks[2], TokenKind::Identifier);
}

#[test]
fn test_newline_is_significant_not_swallowed() {
    let ks = kinds("a\nb");
    assert!(ks.contains(&TokenKind::Newline));
}

#[test]
fn test_horizontal_whitespace_does_not_produce_whitespace_token() {
    let ks = kinds("a b");
    assert!(!ks.contains(&TokenKind::Whitespace));
}

// ── Consecutive operators ─────────────────────────────────────────────────────

#[test]
fn test_not_equal_does_not_consume_extra_equal() {
    // "!= =" → NotEqual, Assign
    let ks = kinds("!= =");
    assert_eq!(ks[0], TokenKind::NotEqual);
    assert_eq!(ks[1], TokenKind::Assign);
}

#[test]
fn test_equal_does_not_consume_extra_equal() {
    // "=== " → Equal, Assign
    let ks = kinds("===");
    assert_eq!(ks[0], TokenKind::Equal);
    assert_eq!(ks[1], TokenKind::Assign);
}

#[test]
fn test_exclamation_alone_is_not() {
    let ks = kinds("!x");
    assert_eq!(ks[0], TokenKind::Not);
    assert_eq!(ks[1], TokenKind::Identifier);
}
