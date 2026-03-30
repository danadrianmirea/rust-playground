fn is_palindrome(s: &str) -> bool {
    let filtered = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    filtered.clone().eq(filtered.rev())
}

fn main() {
    let test_string = "A man, a plan, a canal: Panama";
    if is_palindrome(test_string) {
        println!("\"{}\" is a palindrome.", test_string);
    } else {
        println!("\"{}\" is not a palindrome.", test_string);
    }

    let another_test_string = "madam";
    if is_palindrome(another_test_string) {
        println!("\"{}\" is a palindrome.", another_test_string);
    } else {
        println!("\"{}\" is not a palindrome.", another_test_string);
    }
}