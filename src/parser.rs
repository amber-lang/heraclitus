
pub fn lexer(code: &str) -> Vec<String> {
    const NO_REGION: i32 = -1;
    const regions: [(&str,&str);4] = [("'", "'"), ("\"", "\""), ("/*", "*/"), ("//", "\n")];
    const region_size: i32 = 2;

    let mut region: i32 = NO_REGION;
    let mut tokens = vec![];
    let mut token = String::new();

    // Add token to tokens array
    let add_word = |tokens: &mut Vec<String>, token: String| {
        if token.len() != 0 {
            tokens.push(token)
        }
    };

    // Find the best matching region 
    // with the biggest matching length
    let match_region = |future: &str, region: i32| -> i32 {
        let mut len = 0;
        let mut reg = NO_REGION;
        for (index, current) in regions.iter().enumerate() {
            let left = current.0;
            if left.len() <= len { continue }
            if left.len() > future.len() { continue }
            if region == NO_REGION && left == &future[..left.len()] {
                reg = index as i32;
                len = left.len();
            }
        }
        reg
    };

    for (index, letter) in code.char_indices() {
        // White space case
        if letter == ' ' && region == NO_REGION {
            add_word(&mut tokens, token);
            token = String::new();
        }
        else if region != NO_REGION {
            let reg = regions[region as usize].1;
            // TODO: Finish
            if reg.len() && reg == &code[index..reg.len()] {

            }
            // println!("{}", &code[index..reg.1.len()]);
            // if reg == code[index..reg.len()]
            // TODO: Finish
        }
        else {
            region = match_region(&code[index..], region);
            if region > NO_REGION {
                token.push(letter);
            }
        }
    }
    add_word(&mut tokens, token);
    tokens
}

pub fn tokenizer(raw_tokens: Vec<String>, double_ops: Vec<&str>) {
    let mut tokens = vec![];
    let mut prevs: [&str;2] = ["", ""];
    let mut index = 0;
    for token in raw_tokens.iter() {
        tokens.push(String::from(token));
        prevs[index] = token;
        index += 1;
    }
}