// Don't change this function.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Move `string2` outside the block to extend its lifetime
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz"); // Now lives as long as `string1`
    let result;

    {
        result = longest(&string1, &string2);
    }

    println!("The longest string is '{result}'");
}
