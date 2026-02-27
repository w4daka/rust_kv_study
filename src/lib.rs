use std::collections::HashMap;
#[derive(PartialEq, Debug)]
pub enum Command {
    Set { key: String, value: String },
    Get { key: String },
}

/// コマンド文字列をパースして Command 列挙型を返す
/// 実装に挑戦してみてください！
pub fn parse_command(input: &str) -> Result<Command, String> {
    // ここを実装してください
    let parts: Vec<&str> = input.split_whitespace().collect(); //split_whitespaceは「イテレータ」を返すからそのままではparts[0]なのでこれをVec<&str>に変換する。
    match parts.get(0) {
        Some(&"SET") => {
            if parts.len() < 3 {
                return Err("Not enough arguments".to_string());
            }

            let cmd = Command::Set {
                key: parts[1].to_string(),
                value: parts[2].to_string(),
            };
            Ok(cmd)
        }
        Some(&"GET") => {
            if parts.len() < 2 {
                return Err("Not enough arguments".to_string());
            }
            let cmd = Command::Get {
                key: parts[1].to_string(),
            };
            Ok(cmd)
        }
        _ => Err("Unknown command".to_string()),
    }
}

struct Storage {
    data: HashMap<String, String>,
}

impl Storage {
    fn execute(&mut self, cmd: Command) -> Result<Option<String>, String> {
        match cmd {
            Command::Set { key, value } => Ok(Storage::data.insert(key, value)),
            Command::Get { key } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_strage_set() {}
}
