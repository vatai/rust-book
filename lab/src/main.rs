fn main() {
    println!("Hello, world!");

    let myval = 21;
    let mut mystr = String::from("Hello");
    myfun(myval, &mut mystr);
    println!("myval: {}", myval);
    println!("mystring: {}", mystr);
}

fn myfun(mut val: i32, string: &mut String) {
    println!("val (v1): {}", val);
    println!("string (v1): {}", string);
    val *= 2;
    *string += " darkness my old friend!";
    println!("val (v2): {}", val);
    println!("string (v2): {}", string);
}
