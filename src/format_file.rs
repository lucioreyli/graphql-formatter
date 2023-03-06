use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use graphql_parser::{
    query::{parse_query, Document as QueryDocument},
    schema::{parse_schema, Document as SchemaDocument},
};

pub fn format_file(file_path: &Path) {
    let file_path = PathBuf::from(file_path);
    let mut gql_file = File::options()
        .read(true)
        .open(&file_path)
        .expect("unable to open file");

    let mut file_content = String::new();
    gql_file.read_to_string(&mut file_content).unwrap();

    let formatted_query_file = parse_query::<&str>(&file_content).unwrap_or(QueryDocument {
        definitions: vec![],
    });
    let formatted_schema_file = parse_schema::<&str>(&file_content).unwrap_or(SchemaDocument {
        definitions: vec![],
    });

    let mut gql_file = File::options()
        .write(true)
        .truncate(true)
        .open(file_path)
        .expect("unable to open file");

    let gql_formatted_content = if !formatted_query_file.definitions.is_empty() {
        formatted_query_file.to_string()
    } else {
        formatted_schema_file.to_string()
    };

    gql_file
        .write_all(gql_formatted_content.as_bytes())
        .expect("unable to write file");
}
