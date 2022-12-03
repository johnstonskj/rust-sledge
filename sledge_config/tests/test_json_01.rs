pub mod parse_file;

#[test]
fn test_parse_json_01() {
    println!("json-01: {:#?}", parse_file::parse("json-01"));
}
