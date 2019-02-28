// @url https://leetcode.com/problems/valid-parentheses/

#[allow(dead_code)]
pub fn is_valid_1(s: String) -> bool {
    let mut vec : Vec<char> = vec![];
    let mut oldc : char;
    for ch in s.chars() {
        if vec.len() == 0 { vec.push(ch); }
        else {
            oldc = *vec.last().unwrap();
            if (ch == ')' && oldc != '(') ||
                (ch == ']' && oldc != '[') ||
                (ch == '}' && oldc != '{') {
                    return false
                }
            let ss = format!("{}{}", oldc, ch);
            if ss == "()" || ss == "[]" || ss == "{}" {
                vec.pop(); 
            } else { vec.push(ch); }
        }
    }
    vec.len() == 0
}


// short one
#[allow(dead_code)]
pub fn is_valid(s: String) -> bool {
    let mut vec : Vec<char> = vec![];
    for ch in s.chars() {
        match ch {
            '(' => vec.push(')'),
            '[' => vec.push(']'),
            '{' => vec.push('}'),
            _ => {
                if vec.len() == 0 || vec.pop().unwrap() != ch {
                    return false
                }
            }
        }
    }
    vec.len() == 0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn it_works_01() {
        assert_eq!(is_valid(String::from("()")), true);
    }
    #[test]
    fn it_works_02() {
        assert_eq!(is_valid(String::from("([)]")), false);
    }
    #[test]
    fn it_works_03() {
        assert_eq!(is_valid(String::from("{[]}")), true);
    }
    #[test]
    fn it_works_04() {
        assert_eq!(is_valid(String::from("()[]{}")), true);
    }
}

