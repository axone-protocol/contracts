#[cfg(test)]
pub mod testutil {
    use axone_rdf::owned_model::OwnedQuad;
    use axone_rdf::serde::NQuadsReader;
    use std::env;
    use std::fs::File;
    use std::io::{BufReader, Read};
    use std::path::Path;

    pub fn read_test_quads(file: &str) -> Vec<OwnedQuad> {
        let raw_rdf = read_test_data(file);
        let buffer = BufReader::new(raw_rdf.as_slice());
        let mut reader = NQuadsReader::new(buffer);
        reader.read_all().unwrap()
    }

    pub fn read_test_data(file: &str) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        File::open(
            Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap())
                .join("testdata")
                .join(file),
        )
        .unwrap()
        .read_to_end(&mut bytes)
        .unwrap();

        bytes
    }
}
