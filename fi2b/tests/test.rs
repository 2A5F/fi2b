use fi2b::fi2b;

#[test]
fn test1() {
    let a: &[u8] = &fi2b![1, 2, 1.0, -1.0];
    assert_eq!(
        a,
        [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 63u8, 0u8, 0u8, 128u8, 191u8]
    )
}
