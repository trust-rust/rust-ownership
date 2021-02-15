crate fn ex07_modify() {
    // let x = String::from("try to change me");
    let mut x = String::from("try to change me");
    x.push_str(", fine I will");
    println!("x is '{}'", x);
}
