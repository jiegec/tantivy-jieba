tantivy-jieba
============================

An adapter that bridges between tantivy and jieba-rs.

Usage
===========================

Add dependency `tanticy-jieba` to your `Cargo.toml`.

Example
---------------------------

```
use tantivy::tokenizer::*;
let tokenizer = tantivy_jieba::JiebaTokenizer {};
let mut token_stream = tokenizer.token_stream("测试");
assert_eq!(token_stream.next().unwrap().text, "测试");
assert!(token_stream.next().is_none());
```

Register tantivy tokenizer
---------------------------

```
use tantivy::schema::Schema;
use tantivy::tokenizer::*;
use tantivy::Index;
let tokenizer = tantivy_jieba::JiebaTokenizer {};
let index = Index::create_in_ram(schema);
index.tokenizers()
     .register("jieba", tokenizer);
```
