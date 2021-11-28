
pub fn compare_bytes(actual: &[u8], expected: &[u8]) {
    for i in 0..actual.len().min(expected.len()) {
        if actual[i] != expected[i] {
            panic!("Byte {} (0x{:X}) doesn't match: 0x{:X} != 0x{:X}", i, i, actual[i], expected[i]);
        }
    }
    assert_eq!(actual.len(), expected.len());
}
