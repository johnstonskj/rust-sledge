pub mod parse_file;

#[test]
fn test_parse_json_02() {
    println!("json-02: {:#?}", parse_file::parse("json-02"));
}
