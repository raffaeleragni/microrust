use std::{fs::File, io::Read};

use rsgen_avro::{Generator, Source};

fn main() {
    let mut file = File::open("schema.json").unwrap();
    let mut raw_schema = String::new();
    file.read_to_string(&mut raw_schema).unwrap();

    let source = Source::SchemaStr(&raw_schema);
    let mut file = File::create("src/avro.rs").unwrap();

    let g = Generator::new().unwrap();
    g.gen(&source, &mut file).unwrap();
}
