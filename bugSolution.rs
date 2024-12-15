fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Safe way to access vector elements
    if let Some(&x) = vec.get(1) {
        println!("The value at index 1 is: {}", x);
    } else {
        println!("Index out of bounds");
    }

    //Another safe way using match
    match vec.get(10) {
        Some(x) => println!("The value at index 10 is: {}", x),
        None => println!("Index out of bounds")
    }
} 
