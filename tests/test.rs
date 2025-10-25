use minigrep::{Config, run, search, search_case_insensitive};
use std::{fs, error::Error};

#[test]
fn test_run_reads_file() -> Result<(), Box<dyn Error>> {
    
    let filename = "poem_test.txt";
    let contents = "this is a test\nline two";
    fs::write(filename, contents)?;

    let config = Config {
        query: String::from("test"),
        filename: filename.to_string(),
        case_sensitive: true,
    };

    let result = run(config);

    fs::remove_file(filename)?;

    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_config_new_with_valid_args() {
    
    let args = vec![
        String::from("minigrep"),
        String::from("needle"),
        String::from("haystack.txt"),
    ];

    let config = Config::new(&args).unwrap();

    assert_eq!(config.query, "needle");
    assert_eq!(config.filename, "haystack.txt");
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
        Rust:
        safe, fast, productive.
        Pick three.
        Duct tape.";
    
    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
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
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
);
}
