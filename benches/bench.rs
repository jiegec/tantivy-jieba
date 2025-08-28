use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use std::fs;
use std::hint::black_box;
use tantivy_jieba::JiebaTokenizer;
use tantivy_tokenizer_api::{TokenStream, Tokenizer};

fn load_test_text() -> String {
    fs::read_to_string("benches/weicheng.txt").expect("Failed to read weicheng.txt")
}

fn jieba_tokenizer_benchmark(c: &mut Criterion) {
    let text = load_test_text();
    let text_bytes = text.len() as u64;

    let mut group = c.benchmark_group("jieba_tokenizer");
    group.throughput(Throughput::Bytes(text_bytes));
    group.bench_function("tokenize", |b| {
        b.iter(|| {
            let mut tokenizer = JiebaTokenizer::new();
            let mut token_stream = tokenizer.token_stream(black_box(&text));
            while token_stream.advance() {
                black_box(token_stream.token());
            }
        })
    });

    group.finish();
}

criterion_group!(benches, jieba_tokenizer_benchmark);
criterion_main!(benches);
