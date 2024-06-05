#![no_std]

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use alloc::string::{String, ToString};
use core::str;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Object(Vec<(String, JsonValue)>),
    Array(Vec<JsonValue>),
    String(String),
    Number(f64),
    Bool(bool),
    Null,
}

impl JsonValue {
    pub fn get_object(&self) -> Option<&Vec<(String, JsonValue)>> {
        if let JsonValue::Object(ref obj) = *self {
            Some(obj)
        } else {
            None
        }
    }

    pub fn get_array(&self) -> Option<&Vec<JsonValue>> {
        if let JsonValue::Array(ref array) = *self {
            Some(array)
        } else {
            None
        }
    }

    pub fn get_string(&self) -> Option<&String> {
        if let JsonValue::String(ref s) = *self {
            Some(s)
        } else {
            None
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        if let JsonValue::Number(n) = *self {
            Some(n)
        } else {
            None
        }
    }

    pub fn get_bool(&self) -> Option<bool> {
        if let JsonValue::Bool(b) = *self {
            Some(b)
        } else {
            None
        }
    }

    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        if let JsonValue::Object(ref obj) = *self {
            for (k, v) in obj.iter() {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut JsonValue> {
        if let JsonValue::Object(ref mut obj) = *self {
            for (k, v) in obj.iter_mut() {
                if k == key {
                    return Some(v);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub enum JsonError {
    UnexpectedEnd,
    UnexpectedToken,
    InvalidNumber,
}

pub fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
    let mut chars = input.chars().peekable();
    parse_value(&mut chars)
}

fn parse_value<I>(chars: &mut core::iter::Peekable<I>) -> Result<JsonValue, JsonError>
    where
        I: Iterator<Item = char>,
{
    match chars.peek() {
        Some('{') => parse_object(chars),
        Some('[') => parse_array(chars),
        Some('"') => parse_string(chars),
        Some('t') | Some('f') => parse_bool(chars),
        Some('n') => parse_null(chars),
        Some(_) => parse_number(chars),
        None => Err(JsonError::UnexpectedEnd),
    }
}

fn parse_object<I>(chars: &mut core::iter::Peekable<I>) -> Result<JsonValue, JsonError>
    where
        I: Iterator<Item = char>,
{
    let mut object = Vec::new();
    chars.next(); // Consume '{'

    loop {
        consume_whitespace(chars);
        if let Some('}') = chars.peek() {
            chars.next(); // Consume '}'
            break;
        }

        let key = parse_key(chars)?;
        consume_whitespace(chars);
        if chars.next() != Some(':') {
            return Err(JsonError::UnexpectedToken);
        }
        consume_whitespace(chars);
        let value = parse_value(chars)?;
        object.push((key, value));
        consume_whitespace(chars);
        match chars.next() {
            Some(',') => continue,
            Some('}') => break,
            _ => return Err(JsonError::UnexpectedToken),
        }
    }

    Ok(JsonValue::Object(object))
}

fn parse_key<I>(chars: &mut core::iter::Peekable<I>) -> Result<String, JsonError>
    where
        I: Iterator<Item = char>,
{
    match chars.peek() {
        Some('"') => {
            if let JsonValue::String(s) = parse_string(chars)? {
                Ok(s)
            } else {
                Err(JsonError::UnexpectedToken)
            }
        }
        Some(_) => {
            let mut key = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() || c == ':' {
                    break;
                }
                key.push(c);
                chars.next();
            }
            Ok(key)
        }
        None => Err(JsonError::UnexpectedEnd),
    }
}
fn parse_array<I>(chars: &mut core::iter::Peekable<I>) -> Result<JsonValue, JsonError>
    where
        I: Iterator<Item = char>,
{
    let mut array = Vec::new();
    chars.next(); // Consume '['

    loop {
        consume_whitespace(chars);
        if let Some(']') = chars.peek() {
            chars.next(); // Consume ']'
            break;
        }

        let value = parse_value(chars)?;
        array.push(value);
        consume_whitespace(chars);
        match chars.next() {
            Some(',') => continue,
            Some(']') => break,
            _ => return Err(JsonError::UnexpectedToken),
        }
    }

    Ok(JsonValue::Array(array))
}

fn parse_string<I>(chars: &mut core::iter::Peekable<I>) -> Result<JsonValue, JsonError>
    where
        I: Iterator<Item = char>,
{
    let mut string = String::new();
    chars.next(); // Consume '"'

    while let Some(&c) = chars.peek() {
        match c {
            '"' => {
                chars.next(); // Consume '"'
                break;
            }
            '\\' => {
                chars.next(); // Consume '\'
                if let Some(escaped_char) = chars.next() {
                    string.push(match escaped_char {
                        '"' => '"',
                        '\\' => '\\',
                        '/' => '/',
                        'b' => '\x08',
                        'f' => '\x0C',
                        'n' => '\n',
                        'r' => '\r',
                        't' => '\t',
                        _ => return Err(JsonError::UnexpectedToken),
                    });
                } else {
                    return Err(JsonError::UnexpectedEnd);
                }
            }
            _ => {
                string.push(c);
                chars.next();
            }
        }
    }

    Ok(JsonValue::String(string))
}

fn parse_number<I>(chars: &mut core::iter::Peekable<I>) -> Result<JsonValue, JsonError>
    where
        I: Iterator<Item = char>,
{
    let mut number_str = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_digit(10) || c == '.' || c == '-' || c == '+' || c == 'e' || c == 'E' {
            number_str.push(c);
            chars.next();
        } else {
            break;
        }
    }

    number_str
        .parse::<f64>()
        .map(JsonValue::Number)
        .map_err(|_| JsonError::InvalidNumber)
}

fn parse_bool<I>(chars: &mut core::iter::Peekable<I>) -> Result<JsonValue, JsonError>
    where
        I: Iterator<Item = char>,
{
    let mut bool_str = String::new();
    for _ in 0..4 {
        if let Some(c) = chars.next() {
            bool_str.push(c);
        }
    }

    match bool_str.as_str() {
        "true" => Ok(JsonValue::Bool(true)),
        "fals" => {
            if chars.next() == Some('e') {
                Ok(JsonValue::Bool(false))
            } else {
                Err(JsonError::UnexpectedToken)
            }
        }
        _ => Err(JsonError::UnexpectedToken),
    }
}

fn parse_null<I>(chars: &mut core::iter::Peekable<I>) -> Result<JsonValue, JsonError>
    where
        I: Iterator<Item = char>,
{
    let mut null_str = String::new();
    for _ in 0..4 {
        if let Some(c) = chars.next() {
            null_str.push(c);
        }
    }

    if null_str == "null" {
        Ok(JsonValue::Null)
    } else {
        Err(JsonError::UnexpectedToken)
    }
}

fn consume_whitespace<I>(chars: &mut core::iter::Peekable<I>)
    where
        I: Iterator<Item = char>,
{
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_object() {
        let json = r#"{"name":"Alice中文","age":30}"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Object(vec![
            ("name".to_string(), JsonValue::String("Alice中文".to_string())),
            ("age".to_string(), JsonValue::Number(30.0)),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_array() {
        let json = r#"[1, 2, 3, 4]"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::Number(2.0),
            JsonValue::Number(3.0),
            JsonValue::Number(4.0),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_string() {
        let json = r#""Hello, world!""#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::String("Hello, world!".to_string());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_bool() {
        let json = r#"true"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Bool(true);
        assert_eq!(result, expected);

        let json = r#"false"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Bool(false);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_null() {
        let json = r#"null"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Null;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_number() {
        let json = r#"12345"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Number(12345.0);
        assert_eq!(result, expected);

        let json = r#"-123.45"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Number(-123.45);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_complex() {
        let json = r#"[{"name":"Alice中文","age":30},true,false,null,123,"test"]"#;
        let result = parse_json(json).unwrap();
        let expected = JsonValue::Array(vec![
            JsonValue::Object(vec![
                ("name".to_string(), JsonValue::String("Alice中文".to_string())),
                ("age".to_string(), JsonValue::Number(30.0)),
            ]),
            JsonValue::Bool(true),
            JsonValue::Bool(false),
            JsonValue::Null,
            JsonValue::Number(123.0),
            JsonValue::String("test".to_string()),
        ]);
        assert_eq!(result, expected);
    }


    #[test]
    fn test_long_complex(){
        let json = r#"{
    location: {
        id: "WT3Q0FW9ZJ3Q",
        name: "武汉",
        country: "CN",
        path: "武汉,武汉,湖北,中国",
        timezone: "Asia/Shanghai",
        timezone_offset: "+08:00"
    },
    daily: [
        {
            date: "2024-05-31",
            text_day: "晴",
            code_day: "0",
            text_night: "晴",
            code_night: "1",
            high: "26",
            low: "18",
            rainfall: "0.00",
            precip: "0.00",
            wind_direction: "无持续风向",
            wind_direction_degree: "",
            wind_speed: "8.4",
            wind_scale: "2",
            humidity: "81"
        },
        {
            date: "2024-05-31",
            text_day: "晴",
            code_day: "0",
            text_night: "晴",
            code_night: "1",
            high: "26",
            low: "18",
            rainfall: "0.00",
            precip: "0.00",
            wind_direction: "无持续风向",
            wind_direction_degree: "",
            wind_speed: "8.4",
            wind_scale: "2",
            humidity: "81"
        },
        {
            date: "2024-05-31",
            text_day: "晴",
            code_day: "0",
            text_night: "晴",
            code_night: "1",
            high: "26",
            low: "18",
            rainfall: "0.00",
            precip: "0.00",
            wind_direction: "无持续风向",
            wind_direction_degree: "",
            wind_speed: "8.4",
            wind_scale: "2",
            humidity: "81"
        }
    ]
}
"#;

        let mut is_object = false;
        match parse_json(json){
            Ok(result) => {


                //use get function
                assert_eq!(3 ,result.get("daily").unwrap().get_array().unwrap().len());

                assert_eq!( 6,result.get("location").unwrap().get_object().unwrap().len());


                if let JsonValue::Object(result) = result {
                    is_object = true;
                    for member in result {
                        if member.0 == "location".to_string() {
                            let expected = vec![
                                ("id".to_string(), JsonValue::String("WT3Q0FW9ZJ3Q".to_string())),
                                ("name".to_string(), JsonValue::String("武汉".to_string())),
                                ("country".to_string(), JsonValue::String("CN".to_string())),
                                ("path".to_string(), JsonValue::String("武汉,武汉,湖北,中国".to_string())),
                                ("timezone".to_string(), JsonValue::String("Asia/Shanghai".to_string())),
                                ("timezone_offset".to_string(), JsonValue::String("+08:00".to_string()))
                            ];
                            assert_eq!(*member.1.get_object().unwrap() ,expected );
                        }

                        if member.0 == "daily".to_string() {
                            assert_eq!(3 ,member.1.get_array().unwrap().len() );
                        }
                    }

                }
            }

            Err(e) => {

            }
        }
        assert_eq!(is_object, true);
    }


}
