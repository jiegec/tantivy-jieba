tantivy-jieba
============================

[![Crates.io version][crate-img]][crate]
[![docs.rs][docs-img]][docs]
[![Changelog][changelog-img]][changelog]
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fjiegec%2Ftantivy-jieba.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2Fjiegec%2Ftantivy-jieba?ref=badge_shield)


An adapter that bridges between tantivy and jieba-rs.

Usage
===========================

Add dependency `tantivy-jieba` to your `Cargo.toml`.

Example
---------------------------

```rust
use tantivy::tokenizer::*;
let mut tokenizer = tantivy_jieba::JiebaTokenizer {};
let mut token_stream = tokenizer.token_stream("测试");
assert_eq!(token_stream.next().unwrap().text, "测试");
assert!(token_stream.next().is_none());
```

Register tantivy tokenizer
---------------------------

```rust
use tantivy::schema::Schema;
use tantivy::tokenizer::*;
use tantivy::Index;
let tokenizer = tantivy_jieba::JiebaTokenizer {};
let index = Index::create_in_ram(schema);
index.tokenizers()
     .register("jieba", tokenizer);
```

[crate-img]:     https://img.shields.io/crates/v/tantivy-jieba.svg
[crate]:         https://crates.io/crates/tantivy-jieba
[changelog-img]: https://img.shields.io/badge/changelog-online-blue.svg
[changelog]:     https://github.com/jiegec/tantivy-jieba/blob/master/CHANGELOG.md
[docs-img]:      https://docs.rs/tantivy-jieba/badge.svg
[docs]:          https://docs.rs/tantivy-jieba


## License
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2Fjiegec%2Ftantivy-jieba.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2Fjiegec%2Ftantivy-jieba?ref=badge_large)
