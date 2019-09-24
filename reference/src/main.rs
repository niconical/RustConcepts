fn calculate_length(s: &String) -> usize {  // s is a reference to a string
    s.len()
}   // here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s;
}

fn main() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("the length of '{}' is {}", s1, len);
    
    change(&mut s1);

    {
        let _r1 = &mut s1;
    }

    let _r2 = &s1;
    let _r3 = &s1;
    let _r4 = &mut s1;  // FIXME: maybe changed
    
    let reference_to_nothing = dangle();
}
