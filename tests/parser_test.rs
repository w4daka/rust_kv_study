use rust_kv_study::{Command, parse_command};

#[test]
fn test_parse_set_command() {
    // ここに「SET key value」が正しくパースされることを確認するテストを書いてください
    let input = "SET mykey myvalue";
    let result = parse_command(input);
    let expected = Ok(Command::Set {
        key: "mykey".to_string(),
        value: "myvalue".to_string(),
    }); //成功ケースをテストするときはOKで包む。参考;https://doc.rust-lang.org/std/result/.
    assert_eq!(result, expected);
}

#[test]
fn test_parse_get_command() {
    let input = "GET mykey";
    let result = parse_command(input);
    let expected = Ok(Command::Get {
        key: "mykey".to_string(),
    });
    assert_eq!(result, expected);
}

#[test]
fn test_parse_unknown_command() {
    // ここに、未知のコマンドがエラーになることを確認するテストを書いてください
    let input = "UNKNOWN_CMD";
    let result = parse_command(input);
    assert!(result.is_err()); //errorであることを確認するだけ
    let expected = Err("Unknown command".to_string());
    assert_eq!(result, expected); //error messageの中身まで確認
}

#[test]
fn test_parse_set_not_enough_arguments() {
    let input = "SET mykey";
    let result = parse_command(input);
    assert!(result.is_err());
    let expected = Err("Not enough arguments".to_string());
    assert_eq!(result, expected);
}

#[test]
fn test_parse_get_not_enough_arguments() {
    let input = "GET";
    let result = parse_command(input);
    assert!(result.is_err());
    let expected = Err("Not enough arguments".to_string());
    assert_eq!(result, expected);
}
