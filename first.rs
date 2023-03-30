fn retruning_string() -> String {
    let s = String::from("my string");
    s
}


fn main() {
    // The statements here will be executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
    let x = 5 +  90 +  5;
    //let x = 5 +  5;
    println!("Is `x` 10 or 100? x = {}", x);
    println!("{}",retruning_string());
}
