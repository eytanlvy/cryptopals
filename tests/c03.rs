#[cfg(test)]
mod tests {
    use cryptopals::set01::{hex_to_bytes, single_bytes_xor_cypher};

    #[test]
    fn test_c03() {
        let str1 = hex_to_bytes("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
        let (_key, _res) = single_bytes_xor_cypher(&str1);
        assert_eq!(_key, 88);

    }
}
