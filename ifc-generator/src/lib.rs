use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use thiserror::Error;

use iso_10303::{
    express::{parser, parser::Error as ExpressParseError},
    generator::gencode::generator::Generator,
};

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    #[error("Express parse error")]
    ExpressParseError(#[from] ExpressParseError),
}

pub fn compile_express<P: AsRef<Path>, T: AsRef<Path>>(
    express_file_path: P,
    name: String,
    rust_file_path: T,
) -> Result<(), GeneratorError> {
    let mut express_file = File::open(express_file_path)?;
    let mut express_file_content: Vec<u8> = vec![];
    express_file.read_to_end(&mut express_file_content)?;

    let schema = parser::schema().parse(&express_file_content)?;

    let generator = Generator::new(schema, name);
    let code = generator.gencode();

    let mut rust_file = File::open(rust_file_path)?;
    rust_file.write_all(code.as_bytes())?;

    return Ok(());
}
