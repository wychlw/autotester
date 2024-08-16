use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[macro_export]
macro_rules! todo {
    () => {
        Err(Box::<dyn Error>::from("TODO"))
    };
}

#[macro_export]
macro_rules! unfinished {
    () => {
        Err(Box::<dyn Error>::from("UNFINISHED"))
    };
}

pub fn rand_string(len: usize) -> Vec<u8> {
    let rnd = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<Vec<u8>>();

    rnd
}

pub fn format_string(data: &[u8]) -> String {
    // 对于每个 utf-8，如果是可打印字符，直接转换为字符，否则转换为转义字符
    let data = String::from_utf8(data.to_vec()).unwrap();
    let mut result = String::new();
    for c in data.chars() {
        if c == '\n' {
            result.push_str("\\n");
        } else if c == '\r' {
            result.push_str("\\r");
        } else if c == '\t' {
            result.push_str("\\t");
        } else if c.is_alphanumeric() || c.is_ascii_punctuation() || c.is_whitespace() {
            result.push(c);
        } else {
            result.push_str(&format!("\\u{:04x}", c as u32));
        }
    }
    println!("Format Result: {:#?}", result);
    result
}
