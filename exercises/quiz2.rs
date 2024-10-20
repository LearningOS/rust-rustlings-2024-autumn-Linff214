// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 函数签名需要接收一个 Vec<(String, Command)>，并返回一个 Vec<String>
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output: Vec<String> = vec![]; // 创建一个空的 Vec<String>
        
        // 遍历输入的 Vec<(String, Command)>
        for (string, command) in input.iter() {
            let transformed_string = match command {
                Command::Uppercase => string.to_uppercase(), // 转换为大写
                Command::Trim => string.trim().to_string(),  // 移除两端的空白字符
                Command::Append(times) => {
                    let mut result = string.clone(); // 克隆字符串
                    for _ in 0..*times {
                        result.push_str("bar"); // 追加 "bar"
                    }
                    result
                }
            };
            output.push(transformed_string); // 将处理后的字符串添加到 output 中
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
