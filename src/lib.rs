//! A library that bridges between tantivy and jieba-rs.
//!
//! It implements a [`JiebaTokenizer`](./struct.JiebaTokenizer.html) for the purpose.
#![forbid(unsafe_code)]

use lazy_static::lazy_static;
use tantivy_tokenizer_api::{Token, TokenStream, Tokenizer};

lazy_static! {
    static ref JIEBA: jieba_rs::Jieba = jieba_rs::Jieba::new();
}

/// Tokenize the text using jieba_rs.
///
/// Need to load dict on first tokenization.
///
/// # Example
/// ```rust
/// use tantivy::tokenizer::*;
/// let mut tokenizer = tantivy_jieba::JiebaTokenizer {};
/// let mut token_stream = tokenizer.token_stream("测试");
/// assert_eq!(token_stream.next().unwrap().text, "测试");
/// assert!(token_stream.next().is_none());
/// ```
///
/// # Register tantivy tokenizer
/// ```rust
/// use tantivy::schema::Schema;
/// use tantivy::tokenizer::*;
/// use tantivy::Index;
/// # fn main() {
/// # let schema = Schema::builder().build();
/// let tokenizer = tantivy_jieba::JiebaTokenizer {};
/// let index = Index::create_in_ram(schema);
/// index.tokenizers()
///      .register("jieba", tokenizer);
/// # }
#[derive(Clone)]
pub struct JiebaTokenizer;

/// Token stream instantiated by [`JiebaTokenizer`](./struct.JiebaTokenizer.html).
///
/// Use [`JiebaTokenizer::token_stream`](./struct.JiebaTokenizer.html#impl-Tokenizer<%27a>).
pub struct JiebaTokenStream<'a> {
    text: &'a str,
    jieba_tokens: Vec<jieba_rs::Token<'a>>,
    index: usize,
    token: Token,
}

impl TokenStream for JiebaTokenStream<'_> {
    fn advance(&mut self) -> bool {
        if self.index >= self.jieba_tokens.len() {
            return false;
        }
        let jieba_token = &self.jieba_tokens[self.index];
        let offset_from = jieba_token.word.as_ptr() as usize - self.text.as_ptr() as usize;
        self.token = Token {
            offset_from,
            offset_to: offset_from + jieba_token.word.len(),
            position: self.index,
            text: jieba_token.word.to_string(),
            position_length: 1,
        };
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

impl Tokenizer for JiebaTokenizer {
    type TokenStream<'a> = JiebaTokenStream<'a>;

    fn token_stream<'a>(&mut self, text: &'a str) -> JiebaTokenStream<'a> {
        let jieba_tokens = JIEBA.tokenize(text, jieba_rs::TokenizeMode::Search, true);
        let token = jieba_tokens
            .first()
            .map(|token| Token {
                offset_from: token.word.as_ptr() as usize - text.as_ptr() as usize,
                offset_to: token.word.as_ptr() as usize - text.as_ptr() as usize + token.word.len(),
                position: 0,
                text: token.word.to_string(),
                position_length: 1,
            })
            .unwrap_or_default();
        JiebaTokenStream {
            text,
            jieba_tokens,
            index: 0,
            token,
        }
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
        // check position
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.position, i);
        }
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
