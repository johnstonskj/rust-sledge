pub mod parse_file;

#[test]
fn test_parse_json_03() {
    println!("json-03: {:#?}", parse_file::parse("json-03"));
}
