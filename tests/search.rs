//! Integration test for tantivy-jieba

use tantivy::collector::Count;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::tokenizer::*;
use tantivy::Index;

#[test]
fn search() {
    // Build schema
    let mut schema_builder = Schema::builder();
    let name = schema_builder.add_text_field(
        "name",
        TextOptions::default()
            .set_indexing_options(
                TextFieldIndexing::default()
                    .set_tokenizer("jieba")
                    .set_index_option(IndexRecordOption::WithFreqsAndPositions),
            )
            .set_stored(),
    );
    let schema = schema_builder.build();

    // Register tantivy tokenizer
    let tokenizer = tantivy_jieba::JiebaTokenizer {};
    let index = Index::create_in_ram(schema);
    let analyzer = TextAnalyzer::builder(tokenizer)
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(Stemmer::default())
        .build();
    index.tokenizers().register("jieba", analyzer);

    // Index some documents
    let mut index_writer = index.writer(50_000_000).unwrap();
    index_writer
        .add_document(doc!(name => "中华人民共和国人民大会堂"))
        .unwrap();
    index_writer.commit().unwrap();

    // Search keywords
    let reader = index.reader().unwrap();
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![name]);

    for query_str in [
        "中华",
        "人民",
        "共和国",
        "华人",
        "共和",
        "人民共和",
        "中华人民共和国",
        "共和国人民",
        "人民大会堂",
    ] {
        let query = query_parser.parse_query(query_str).unwrap();
        println!("query: {query:?}");
        let count = searcher.search(&query, &Count).unwrap();
        assert_eq!(count, 1, "doc not found for query: {query_str}");
    }
}
