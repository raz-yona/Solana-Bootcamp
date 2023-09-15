fn main() {
    let string1 = String::from("string1 ");
    let string2 = String::from("string2 ");
    let concatenated_string = concatenate_strings(&string1,&string2);
    println!("{}",concatenated_string);
}

fn concatenate_strings(s1: &String, s2: &String) -> String {
    let mut result = String::from("");
    result.push_str(s1);
    result.push_str(s2);
    return result;
}