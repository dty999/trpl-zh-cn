fn main() {
    println!("Calling function defined below:");
    my_function();
    another_function(5);
}

fn my_function() {
    println!("This function was defined after main!");
}

fn another_function(x: i32) {
    println!("This function takes parameter: {}", x);
}