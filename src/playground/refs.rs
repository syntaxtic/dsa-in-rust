pub fn go() {
    let var = 9;
    let var_ref = &var;

    println!("{:?}", (var, &var, var_ref, &var_ref, *var_ref));

    assert!(var == *var_ref);
    assert!(&var == var_ref);

    println!("all asserted!");
}
