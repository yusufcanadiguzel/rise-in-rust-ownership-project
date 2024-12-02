fn main() {
    let string1 = "hello";
    let string2 = " world";

    let result = concatenate_strings(&string1, &string2);

    println!("{}", result);
}

fn concatenate_strings(first_input: &str, second_input: &str) -> String {
    let mut result: String = String::new();
    result.push_str(first_input);
    result.push_str(second_input);
    result
}