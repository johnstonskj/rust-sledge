pub mod parse_file;

#[test]
fn test_parse_json_04() {
    println!("json-04: {:#?}", parse_file::parse("json-04"));
}
