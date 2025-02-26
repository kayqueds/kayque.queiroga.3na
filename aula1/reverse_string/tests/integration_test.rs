use reverse_string::reverse; // Importa a função da biblioteca principal

#[test]
fn test_reverse_normal_string() {
    let input = "Data Structures";
    let expected = "serutcurtS ataD";
    assert_eq!(reverse(input), expected);
}

#[test]
fn test_reverse_empty_string() {
    let input = "";
    let expected = "";
    assert_eq!(reverse(input), expected);
}

#[test]
fn test_reverse_single_character() {
    let input = "A";
    let expected = "A";
    assert_eq!(reverse(input), expected);
}
