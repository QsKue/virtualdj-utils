
#[test]
#[ignore]
fn test_db() {
    let db = virtualdj_utils::database::parse_database(r#"D:\VirtualDJ\database.xml"#).unwrap();
    virtualdj_utils::database::write_database(r#"D:\VirtualDJ\database.gen.xml"#, &db).unwrap();
}