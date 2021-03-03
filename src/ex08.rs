crate fn ex08_mutable_reference() {
    let mut x = String::from("try to change me");
    change_it(&mut x);
    println!("x is '{}'", x);
    blah();
}

fn change_it(x: &mut String) {
    x.push_str(", fine I will");
}
