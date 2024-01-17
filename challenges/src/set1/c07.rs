#[cfg(test)]
mod tests {
    use serialize::{decode_base64};
	use std::path::Path;
	use std::fs;

    #[test]
    fn test_c07() {
		let key = b"YELLOW SUBMARINE";
		let path = Path::new("data/c07.txt");
		let content = fs::read_to_string(path)
			.expect("Should have been able to read the file");
		let decoded_content = decode_base64(&content).unwrap();
	}
}