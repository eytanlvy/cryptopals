#[cfg(test)]
mod tests {
    use xor::detect_sc_xor;
	use std::path::Path;

    #[test]
    fn test_c04() {
		let path = Path::new("data/c04.txt");
        let (_key, _res) = detect_sc_xor(&path);
        assert_eq!(_key, 53);
    }
}