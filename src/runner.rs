fn test_spades() {
    let out = Command::new("spades")
        .arg("--test")
        .output()
        .unwrap();

    println!("{:?}", out.status);
}