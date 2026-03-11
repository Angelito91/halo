// The Halo Programming Language
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::Position;

/// Converts a flat string of Halo source code into a sequence of [`Token`]s.
///
/// The lexer is a single-pass, hand-written scanner.  It does **not** produce
/// `Whitespace` or `Comment` tokens — those are consumed silently.  The only
/// whitespace that reaches the parser is `Newline`, which acts as a statement
/// terminator (Halo has no semicolons).
pub struct Lexer {
    /// Source characters stored as a `Vec<char>` so we get O(1) indexed access
    /// without having to deal with UTF-8 byte offsets.
    chars: Vec<char>,
    /// Index of the *next* character to be consumed.
    cursor: usize,
    /// Current source line (1-based).
    line: u32,
    /// Current source column (1-based).
    column: u32,
}

impl Lexer {
    /// Create a new `Lexer` for the given source string.
    pub fn new(source: String) -> Self {
        Self {
            chars: source.chars().collect(),
            cursor: 0,
            line: 1,
            column: 1,
        }
    }

    // ── Character navigation ──────────────────────────────────────────────────

    /// The character at the current cursor position, or `None` at EOF.
    pub fn current_char(&self) -> Option<char> {
        self.chars.get(self.cursor).copied()
    }

    /// The character *after* the current one, without advancing. Used for
    /// two-character lookahead (e.g. `==`, `//`).
    pub fn peek_char(&self) -> Option<char> {
        self.chars.get(self.cursor + 1).copied()
    }

    /// Consume and return the current character, advancing the cursor and
    /// updating line/column tracking.  Returns `None` at EOF.
    pub fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.get(self.cursor).copied()?;
        self.cursor += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    /// Snapshot of the current source position.
    fn current_position(&self) -> Position {
        Position {
            line: self.line,
            column: self.column,
        }
    }

    // ── Whitespace / comment skipping ─────────────────────────────────────────

    /// Skip horizontal whitespace (spaces, tabs, etc.) but **not** newlines,
    /// because `\n` is significant as a statement terminator.
    pub fn skip_whitespace(&mut self) {
        while matches!(self.current_char(), Some(c) if c != '\n' && c.is_whitespace()) {
            self.next_char();
        }
    }

    /// Skip from the second `/` of a `//` comment to the end of the line.
    /// The trailing `\n` is left in place so the main loop emits a `Newline`
    /// token.
    fn skip_line_comment(&mut self) {
        // Both leading slashes have already been peeked; consume them.
        self.next_char(); // first  /
        self.next_char(); // second /
        while !matches!(self.current_char(), None | Some('\n')) {
            self.next_char();
        }
    }

    // ── Keyword table ─────────────────────────────────────────────────────────

    /// Map a scanned identifier string to its keyword [`TokenKind`], or
    /// return `None` if it is a plain user identifier.
    fn keyword_kind(word: &str) -> Option<TokenKind> {
        match word {
            "if" => Some(TokenKind::If),
            "else" => Some(TokenKind::Else),
            "while" => Some(TokenKind::While),
            "return" => Some(TokenKind::Return),
            "true" => Some(TokenKind::True),
            "false" => Some(TokenKind::False),
            "break" => Some(TokenKind::Break),
            "continue" => Some(TokenKind::Continue),
            "and" => Some(TokenKind::And),
            "or" => Some(TokenKind::Or),
            "not" => Some(TokenKind::Not),
            _ => None,
        }
    }

    // ── Token constructors ────────────────────────────────────────────────────

    /// Emit a single-character token, consuming that character.
    fn single_char_token(&mut self, kind: TokenKind, ch: char) -> Token {
        let pos = self.current_position();
        self.next_char();
        Token::new(kind, ch.to_string(), pos)
    }

    /// Emit a two-character token (e.g. `==`).  Both characters must already
    /// have been consumed by the caller.
    fn two_char_token(kind: TokenKind, lexeme: &'static str, pos: Position) -> Token {
        Token::new(kind, lexeme.to_string(), pos)
    }

    // ── String literal scanner ────────────────────────────────────────────────

    /// Scan a double-quoted string literal.  The opening `"` must have been
    /// consumed before calling this method.
    ///
    /// Supported escape sequences:
    ///
    /// | Escape | Meaning         |
    /// |--------|-----------------|
    /// | `\\`   | Backslash       |
    /// | `\"`   | Double quote    |
    /// | `\n`   | Line feed       |
    /// | `\t`   | Horizontal tab  |
    /// | `\r`   | Carriage return |
    ///
    /// Any other `\x` sequence is passed through unchanged (`\x`).
    ///
    /// An unterminated string (EOF before the closing `"`) produces a
    /// [`TokenKind::Unknown`] token so the parser can emit a helpful error.
    fn scan_string_literal(&mut self, start_pos: Position) -> Token {
        let mut value = String::new();

        loop {
            match self.current_char() {
                // EOF before closing quote — surface the error in the parser.
                None => return Token::new(TokenKind::Unknown, value, start_pos),

                Some('"') => {
                    self.next_char(); // consume closing `"`
                    break;
                }

                Some('\\') => {
                    self.next_char(); // consume backslash
                    let escaped = match self.current_char() {
                        Some('\\') => '\\',
                        Some('"') => '"',
                        Some('n') => '\n',
                        Some('t') => '\t',
                        Some('r') => '\r',
                        // Unknown escape: emit the backslash and the raw char.
                        Some(c) => {
                            self.next_char();
                            value.push('\\');
                            value.push(c);
                            continue;
                        }
                        // EOF directly after backslash.
                        None => break,
                    };
                    self.next_char();
                    value.push(escaped);
                }

                // Embedded newline — include it verbatim and keep scanning.
                Some('\n') => {
                    self.next_char();
                    value.push('\n');
                }

                Some(c) => {
                    self.next_char();
                    value.push(c);
                }
            }
        }

        Token::new(TokenKind::StringLit, value, start_pos)
    }

    // ── Main tokenizer ────────────────────────────────────────────────────────

    /// Scan and return the next token from the source.
    ///
    /// Whitespace (except newlines) and line comments are consumed silently
    /// before each token is produced.  Call this in a loop until
    /// [`TokenKind::Eof`] is returned.
    pub fn next_token(&mut self) -> Token {
        // Consume any horizontal whitespace and `//` comments before deciding
        // what token to emit next.
        loop {
            self.skip_whitespace();
            if self.current_char() == Some('/') && self.peek_char() == Some('/') {
                self.skip_line_comment();
            } else {
                break;
            }
        }

        let pos = self.current_position();

        match self.current_char() {
            // ── End of file ───────────────────────────────────────────────────
            None => Token::new(TokenKind::Eof, String::new(), pos),

            // ── Identifiers and keywords ──────────────────────────────────────
            Some(c) if c.is_alphabetic() || c == '_' => {
                let mut ident = String::new();
                while matches!(self.current_char(), Some(c) if c.is_alphanumeric() || c == '_') {
                    // SAFETY: we just matched Some(c), so next_char() is Some.
                    ident.push(self.next_char().unwrap());
                }
                let kind = Self::keyword_kind(&ident).unwrap_or(TokenKind::Identifier);
                Token::new(kind, ident, pos)
            }

            // ── Numeric literals (integer and float) ──────────────────────────
            Some(c) if c.is_ascii_digit() => {
                let mut num = String::new();
                let mut has_dot = false;
                loop {
                    match self.current_char() {
                        Some(d) if d.is_ascii_digit() => {
                            num.push(d);
                            self.next_char();
                        }
                        // A `.` introduces the fractional part only when:
                        //   • we haven't seen a dot yet, and
                        //   • the character after the dot is also a digit
                        //     (to avoid consuming method-call dots like `x.len()`).
                        Some('.')
                            if !has_dot && self.peek_char().is_some_and(|p| p.is_ascii_digit()) =>
                        {
                            has_dot = true;
                            num.push('.');
                            self.next_char();
                        }
                        _ => break,
                    }
                }
                Token::new(TokenKind::Number, num, pos)
            }

            // ── String literals ───────────────────────────────────────────────
            Some('"') => {
                self.next_char(); // consume opening `"`
                self.scan_string_literal(pos)
            }

            // ── Two-character operators ───────────────────────────────────────
            Some('=') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::Equal, "==", pos)
                } else {
                    Token::new(TokenKind::Assign, "=".to_string(), pos)
                }
            }
            Some('!') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::NotEqual, "!=", pos)
                } else {
                    Token::new(TokenKind::Not, "!".to_string(), pos)
                }
            }
            Some('<') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::LessEqual, "<=", pos)
                } else {
                    Token::new(TokenKind::Less, "<".to_string(), pos)
                }
            }
            Some('>') => {
                self.next_char();
                if self.current_char() == Some('=') {
                    self.next_char();
                    Self::two_char_token(TokenKind::GreaterEqual, ">=", pos)
                } else {
                    Token::new(TokenKind::Greater, ">".to_string(), pos)
                }
            }
            Some('&') => {
                self.next_char();
                if self.current_char() == Some('&') {
                    self.next_char();
                    Self::two_char_token(TokenKind::And, "&&", pos)
                } else {
                    Token::new(TokenKind::Unknown, "&".to_string(), pos)
                }
            }
            Some('|') => {
                self.next_char();
                if self.current_char() == Some('|') {
                    self.next_char();
                    Self::two_char_token(TokenKind::Or, "||", pos)
                } else {
                    Token::new(TokenKind::Unknown, "|".to_string(), pos)
                }
            }

            // ── Single-character tokens ───────────────────────────────────────
            Some('+') => self.single_char_token(TokenKind::Plus, '+'),
            Some('-') => self.single_char_token(TokenKind::Minus, '-'),
            Some('*') => self.single_char_token(TokenKind::Star, '*'),
            Some('/') => self.single_char_token(TokenKind::Slash, '/'),
            Some('%') => self.single_char_token(TokenKind::Modulo, '%'),
            Some('(') => self.single_char_token(TokenKind::LeftParen, '('),
            Some(')') => self.single_char_token(TokenKind::RightParen, ')'),
            Some('{') => self.single_char_token(TokenKind::LeftBrace, '{'),
            Some('}') => self.single_char_token(TokenKind::RightBrace, '}'),
            Some('[') => self.single_char_token(TokenKind::LeftBracket, '['),
            Some(']') => self.single_char_token(TokenKind::RightBracket, ']'),
            Some(':') => self.single_char_token(TokenKind::Colon, ':'),
            Some(',') => self.single_char_token(TokenKind::Comma, ','),
            Some('.') => self.single_char_token(TokenKind::Dot, '.'),

            // ── Newline — statement terminator ────────────────────────────────
            Some('\n') => {
                self.next_char();
                Token::new(TokenKind::Newline, "\n".to_string(), pos)
            }

            // ── Unrecognised character ────────────────────────────────────────
            Some(c) => {
                self.next_char();
                Token::new(TokenKind::Unknown, c.to_string(), pos)
            }
        }
    }
}
