use super::*;

#[test]
fn test_generate_random_temp_path() {
    let a = generate_random_temp_path();
    let b = generate_random_temp_path();

    assert_ne!(a, b);
}
