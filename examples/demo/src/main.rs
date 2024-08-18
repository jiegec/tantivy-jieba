use tantivy::collector::TopDocs;
use tantivy::doc;
use tantivy::query::QueryParser;
use tantivy::schema::{IndexRecordOption, Schema, TextFieldIndexing, TextOptions, Value};
use tantivy::tokenizer::*;
use tantivy::Index;
use tantivy::TantivyDocument;

fn main() {
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
    index_writer.add_document(doc!(
      name => "张华考上了北京大学；李萍进了中等技术学校；我在百货公司当售货员：我们都有光明的前途",
    )).unwrap();
    index_writer.commit().unwrap();

    // Search keywords
    let reader = index.reader().unwrap();
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![name]);
    let query = query_parser.parse_query("售货员").unwrap();
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10)).unwrap();
    println!("Search Result:");
    for (_, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address).unwrap();
        let val = retrieved_doc.get_first(name).unwrap();
        let res = val.as_str().unwrap_or_default().to_string();
        println!("{res}");
        assert_eq!(
            res,
            *"张华考上了北京大学；李萍进了中等技术学校；我在百货公司当售货员：我们都有光明的前途"
        );
    }
}

#[test]
fn test() {
    main();
}
