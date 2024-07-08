pub fn go() {
    let nums = vec![13, 21];

    for num in &nums {
        println!("{:?}", (num, &num, *num));
    }

    for &num in &nums {
        println!("{:?}", (num, &num));
    }

    println!("{:?}", nums);

    for num in nums {
        println!("{:?}", (num, &num));
    }

    // println!("{:?}", nums); // XXX

    println!("all asserted!");
}
