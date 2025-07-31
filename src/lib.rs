//! A library that bridges between tantivy and jieba-rs.
//!
//! It implements a [`JiebaTokenizer`](JiebaTokenizer) and [`CustomJiebaTokenizer`](CustomJiebaTokenizer) for the purpose.
#![forbid(unsafe_code)]

use lazy_static::lazy_static;
use tantivy_tokenizer_api::{Token, TokenStream, Tokenizer};

/// Re-export jieba_rs to handle potential jieba_rs crate version mismatch
pub use jieba_rs;

lazy_static! {
    /// Global [`Jieba`](jieba_rs::Jieba) instance
    static ref JIEBA: jieba_rs::Jieba = jieba_rs::Jieba::new();
}

/// Tokenize the text using jieba_rs.
///
/// Need to load dict on first tokenization.
///
/// # Examples
///
/// ## Independent usage
///
/// ```rust
/// use tantivy::tokenizer::*;
/// let mut tokenizer = tantivy_jieba::JiebaTokenizer {};
/// let mut token_stream = tokenizer.token_stream("测试");
/// assert_eq!(token_stream.next().unwrap().text, "测试");
/// assert!(token_stream.next().is_none());
/// ```
///
/// ## Register as tantivy tokenizer
///
/// ```rust
/// use tantivy::schema::Schema;
/// use tantivy::tokenizer::*;
/// use tantivy::Index;
/// # let schema = Schema::builder().build();
/// let tokenizer = tantivy_jieba::JiebaTokenizer {};
/// let index = Index::create_in_ram(schema);
/// index.tokenizers()
///      .register("jieba", tokenizer);
#[derive(Clone)]
pub struct JiebaTokenizer;

impl Tokenizer for JiebaTokenizer {
    type TokenStream<'str> = JiebaTokenStream<'str>;

    fn token_stream<'str>(&mut self, text: &'str str) -> JiebaTokenStream<'str> {
        token_stream_common(&JIEBA, text)
    }
}

/// Tokenize the text using jieba_rs with custom [`Jieba`](jieba_rs::Jieba) instance.
///
/// Need to load dict on first tokenization.
///
/// # Examples
///
/// ```rust
/// use tantivy::tokenizer::*;
/// let mut jieba = jieba_rs::Jieba::new();
/// let mut tokenizer = tantivy_jieba::CustomJiebaTokenizer::new(jieba);
/// let mut token_stream = tokenizer.token_stream("测试");
/// assert_eq!(token_stream.next().unwrap().text, "测试");
/// assert!(token_stream.next().is_none());
/// ```
#[derive(Clone)]
pub struct CustomJiebaTokenizer {
    jieba: jieba_rs::Jieba,
}

impl CustomJiebaTokenizer {
    /// Create a new [`CustomJiebaTokenizer`](CustomJiebaTokenizer) instance using the given [`Jieba`](jieba_rs::Jieba) instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tantivy::tokenizer::*;
    /// let mut jieba = jieba_rs::Jieba::new();
    /// let mut tokenizer = tantivy_jieba::CustomJiebaTokenizer::new(jieba);
    /// let mut token_stream = tokenizer.token_stream("测试");
    /// assert_eq!(token_stream.next().unwrap().text, "测试");
    /// assert!(token_stream.next().is_none());
    /// ```
    pub fn new(jieba: jieba_rs::Jieba) -> Self {
        Self { jieba }
    }

    /// Extract the [`Jieba`](jieba_rs::Jieba) instance and drop self.
    pub fn into_jieba(self) -> jieba_rs::Jieba {
        self.jieba
    }
}

impl Tokenizer for CustomJiebaTokenizer {
    type TokenStream<'str> = JiebaTokenStream<'str>;

    fn token_stream<'str>(&mut self, text: &'str str) -> JiebaTokenStream<'str> {
        token_stream_common(&self.jieba, text)
    }
}

/// Token stream instantiated by [`JiebaTokenizer`](JiebaTokenizer) or [`CustomJiebaTokenizer`](CustomJiebaTokenizer).
///
/// Use [`JiebaTokenizer::token_stream`](JiebaTokenizer::token_stream) or [`CustomJiebaTokenizer::token_stream`](CustomJiebaTokenizer::token_stream).
pub struct JiebaTokenStream<'str> {
    text: &'str str,
    jieba_tokens: Vec<jieba_rs::Token<'str>>,
    index: usize,
    token: Token,
}

impl TokenStream for JiebaTokenStream<'_> {
    fn advance(&mut self) -> bool {
        if self.index >= self.jieba_tokens.len() {
            return false;
        }
        let jieba_token = &self.jieba_tokens[self.index];
        self.token.offset_from = jieba_token.word.as_ptr() as usize - self.text.as_ptr() as usize;
        self.token.offset_to = self.token.offset_from + jieba_token.word.len();
        self.token.position = jieba_token.start;
        self.token.position_length = jieba_token.end - jieba_token.start;
        self.token.text.clear(); // avoid realloc
        self.token.text.push_str(jieba_token.word);
        self.index += 1;
        true
    }

    fn token(&self) -> &Token {
        &self.token
    }

    fn token_mut(&mut self) -> &mut Token {
        &mut self.token
    }
}

/// Create token stream from text
fn token_stream_common<'str>(jieba: &jieba_rs::Jieba, text: &'str str) -> JiebaTokenStream<'str> {
    let jieba_tokens = jieba.tokenize(text, jieba_rs::TokenizeMode::Search, true);
    let token = jieba_tokens
        .first()
        .map(|token| Token {
            offset_from: token.word.as_ptr() as usize - text.as_ptr() as usize,
            offset_to: token.word.as_ptr() as usize - text.as_ptr() as usize + token.word.len(),
            text: token.word.to_string(),
            position: token.start,
            position_length: token.end - token.start,
        })
        .unwrap_or_default();
    JiebaTokenStream {
        text,
        jieba_tokens,
        index: 0,
        token,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use tantivy_tokenizer_api::{TokenStream, Tokenizer};

        let mut tokenizer = crate::JiebaTokenizer {};
        let mut token_stream = tokenizer.token_stream(
            "张华考上了北京大学；李萍进了中等技术学校；我在百货公司当售货员：我们都有光明的前途",
        );
        let mut tokens = Vec::new();
        let mut token_text = Vec::new();
        while let Some(token) = token_stream.next() {
            tokens.push(token.clone());
            token_text.push(token.text.clone());
        }
        // offset should be byte-indexed
        assert_eq!(tokens[0].offset_from, 0);
        assert_eq!(tokens[0].offset_to, "张华".len());
        assert_eq!(tokens[1].offset_from, "张华".len());
        // check tokenized text
        assert_eq!(
            token_text,
            vec![
                "张华",
                "考上",
                "了",
                "北京",
                "大学",
                "北京大学",
                "；",
                "李萍",
                "进",
                "了",
                "中等",
                "技术",
                "术学",
                "学校",
                "技术学校",
                "；",
                "我",
                "在",
                "百货",
                "公司",
                "百货公司",
                "当",
                "售货",
                "货员",
                "售货员",
                "：",
                "我们",
                "都",
                "有",
                "光明",
                "的",
                "前途"
            ]
        );
    }
}
