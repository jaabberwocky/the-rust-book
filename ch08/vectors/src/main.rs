fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(4);
    println!("v = {:?}", v);

    // we can also use the macro to create a vector
    // will also infer type

    let v2: Vec<i32> = vec![1, 2, 3];
    println!("v2 = {:?}", v2);

    // reading elements
    let v3: Vec<i32> = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    // another way
    let third_2: Option<&i32> = v3.get(2);
    match third_2 {
        Some(third_2) => println!("The third element is {}", third_2),
        None => println!("There is no third element"),
    }

    // simulate an access out of index
    let v3: Vec<i32> = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v3[100]; -> will panic
    let does_not_exist_2 = v3.get(100);
    match does_not_exist_2 {
        Some(does_not_exist_2) => println!("The element is {}", does_not_exist_2),
        None => println!("There is no element"),
    }

    // iterating over the values in a vector
    println!("Iterating over the values in a vector");
    let v4: Vec<i32> = vec![100, 32, 57];

    for n_ref in &v4 {
        println!("{}", n_ref);
    }

    let mut v5: Vec<i32> = vec![100, 32, 57];
    for n_ref in &mut v5 {
        *n_ref += 1;
    }
    println!("v5 = {:?}", v5);
}
