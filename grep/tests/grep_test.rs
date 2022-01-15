#[test]
fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
save, fast, productive.
Pink three.";

    assert_eq!(
        vec!["save, fast, productive."],
        grep::search(query, contents)
    )
}


#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:","Trust me."],
        grep::search_case_insensitive(query,contents)
    )
}