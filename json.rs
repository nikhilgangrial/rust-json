use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::prelude::*;
use std::iter::Peekable;
use std::ops::{Index, IndexMut};

fn skip_whitespaces(json: &mut Peekable<std::str::Chars>) {
    while let Some(&c) = json.peek() {
        if c == ' ' || c == '\n' || c == '\t' {
            json.next();
        } else {
            break;
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Num {
    Integer(i128),
    Float(f64),
}

impl fmt::Display for Num {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Num::Integer(i) => write!(f, "{}", i),
            Num::Float(fl) => write!(f, "{}", fl),
        }
    }
}

impl Eq for Num {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Num {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Num::Integer(i) => i.hash(state),
            Num::Float(fl) => fl.to_bits().hash(state),
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub enum JsonDtype {
    String(String),
    Number(Num),
    Object(Json),
    Array(Vec<JsonDtype>),
    Boolean(bool),
    Null,
}

#[allow(dead_code)]
impl JsonDtype {
    pub fn new<T>(value: T) -> Self
    where
        T: Into<JsonDtype>,
    {
        value.into()
    }

    pub fn stringify_pretty(&self, indent: usize, inc: usize) -> String {
        match self {
            JsonDtype::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
            JsonDtype::Number(n) => format!("{}", n),
            JsonDtype::Object(obj) => format!("{}", obj._stringify_pretty(indent, inc)),
            JsonDtype::Array(arr) => {
                if arr.len() == 0 {
                    return "[]".to_string();
                }
                let mut s = String::from("[\n");
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        s.push_str(",\n");
                    }
                    s.push_str(&format!(
                        "{:indent$}{}",
                        "",
                        item.stringify_pretty(indent + inc, inc),
                        indent = indent + inc
                    ))
                }
                s.push_str(&format!("\n{:indent$}]", "", indent = indent));
                s
            }
            JsonDtype::Boolean(b) => format!("{}", b),
            JsonDtype::Null => format!("null"),
        }
    }
}

impl From<String> for JsonDtype {
    fn from(value: String) -> Self {
        JsonDtype::String(value)
    }
}

impl From<&str> for JsonDtype {
    fn from(value: &str) -> Self {
        JsonDtype::String(value.to_owned())
    }
}

impl From<i128> for JsonDtype {
    fn from(value: i128) -> Self {
        JsonDtype::Number(Num::Integer(value))
    }
}

impl From<f64> for JsonDtype {
    fn from(value: f64) -> Self {
        JsonDtype::Number(Num::Float(value))
    }
}

impl From<Json> for JsonDtype {
    fn from(value: Json) -> Self {
        JsonDtype::Object(value)
    }
}

impl From<Vec<JsonDtype>> for JsonDtype {
    fn from(value: Vec<JsonDtype>) -> Self {
        JsonDtype::Array(value)
    }
}

impl From<bool> for JsonDtype {
    fn from(value: bool) -> Self {
        JsonDtype::Boolean(value)
    }
}

impl From<()> for JsonDtype {
    fn from(_: ()) -> Self {
        JsonDtype::Null
    }
}

impl fmt::Display for JsonDtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JsonDtype::String(s) => {
                write!(f, "\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
            }
            JsonDtype::Number(n) => write!(f, "{}", n),
            JsonDtype::Object(obj) => write!(f, "{}", obj),
            JsonDtype::Array(arr) => {
                write!(f, "[")?;
                for (i, item) in arr.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, "]")
            }
            JsonDtype::Boolean(b) => write!(f, "{}", b),
            JsonDtype::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Json {
    map: HashMap<JsonDtype, JsonDtype>,
}

impl Hash for Json {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.map {
            key.hash(state);
            value.hash(state);
        }
    }
}

#[allow(dead_code)]
impl Json {
    pub fn new() -> Self {
        Json {
            map: HashMap::new(),
        }
    }

    pub fn get<K>(&self, key: K) -> Option<&JsonDtype>
    where
        K: Into<JsonDtype>,
    {
        self.map.get(&key.into())
    }

    pub fn insert<K, V>(&mut self, key: K, value: V)
    where
        K: Into<JsonDtype>,
        V: Into<JsonDtype>,
    {
        self.map.insert(key.into(), value.into());
    }

    pub fn remove<K>(&mut self, key: K)
    where
        K: Into<JsonDtype>,
    {
        self.map.remove(&key.into());
    }

    pub fn keys(&self) -> Vec<&JsonDtype> {
        self.map.keys().collect()
    }

    pub fn stringify(&self) -> String {
        let mut res = String::new();

        res.push('{');
        for (i, (key, value)) in self.iter().enumerate() {
            if i > 0 {
                res.push_str(", ");
            }
            match key {
                JsonDtype::String(_) => res.push_str(format!("{}: {}", key, value).as_str()),
                _ => res.push_str(format!("\"{}\": {}", key, value).as_str()),
            }
        }
        res.push('}');
        res
    }

    pub fn stringify_pretty(&self) -> String {
        self._stringify_pretty(0, 4)
    }

    fn _stringify_pretty(&self, indent: usize, inc: usize) -> String {
        
        if self.is_empty() {
            return "{}".to_string();
        }

        let mut res = String::new();

        res.push_str(format!("{}\n", "{").as_str());

        for (i, (key, value)) in self.iter().enumerate() {
            if i > 0 {
                res.push_str(",\n");
            }
            res.push_str(format!("{:indent$}", "", indent = indent + inc).as_str());
            match key {
                JsonDtype::String(_) => res.push_str(
                    format!("{}: {}", key, value.stringify_pretty(indent + inc, inc)).as_str(),
                ),
                _ => res.push_str(
                    format!("\"{}\": {}", key, value.stringify_pretty(indent + inc, inc)).as_str(),
                ),
            }
        }

        res.push_str(format!("\n{:indent$}{}", "", "}", indent = indent).as_str());
        res
    }

    pub fn parse(json: &str) -> Json {
        let mut json = json.chars().peekable();
        skip_whitespaces(&mut json);

        match json.peek() {
            Some(&'{') => Json::parse_object(&mut json),
            Some(&'[') => {
                let mut res = Json::new();
                res.insert("data", Json::parse_array(&mut json));
                res
            }
            _ => {
                panic!("{}", "unexpected char expected '{'")
            }
        }
    }

    fn parse_value(json: &mut Peekable<std::str::Chars>) -> JsonDtype {
        skip_whitespaces(json);

        match json.peek() {
            Some(&'"') => Json::parse_string(json),
            Some(&('0'..='9')) | Some(&'-') => Json::parse_number(json),
            Some(&'t') | Some(&'f') => Json::parse_boolean(json),
            Some(&'n') => Json::parse_null(json),
            Some(&'[') => Json::parse_array(json),
            Some(&'{') => Json::parse_object(json).into(),
            _ => panic!(
                "{} '{}'",
                "expected '\"' or '0'..='9' or 't' or 'f' or 'n' or '[' or '{' found ",
                json.peek().unwrap()
            ),
        }
    }

    fn parse_string(json: &mut Peekable<std::str::Chars>) -> JsonDtype {
        let mut string = String::new();
        json.next();
        let mut skip: bool = false;
        while let Some(&ch) = json.peek() {
            match ch {
                '\\' => {
                    if skip {
                        string.push(ch);
                        skip = false;
                        json.next();
                        continue;
                    }
                    skip = true;
                    json.next();
                }
                '"' => {
                    if skip {
                        string.push(ch);
                        skip = false;
                        json.next();
                        continue;
                    }
                    json.next();
                    return JsonDtype::String(string);
                }
                _ => {
                    skip = false;
                    string.push(ch);
                    json.next();
                }
            }
        }
        panic!("unexpected char expected '\"' found 'EOF'");
    }

    fn parse_number(json: &mut Peekable<std::str::Chars>) -> JsonDtype {
        let mut number = String::new();
        let mut is_float = false;
        let mut is_exp = false;

        if json.peek().unwrap() == &'-' {
            number.push('-');
            json.next();
        }

        while let Some(&ch) = json.peek() {
            match ch {
                '.' => {
                    number.push(ch);
                    if is_float || is_exp {
                        panic!("unexpected char found {} expected valid", number);
                    }
                    json.next();
                    is_float = true;
                }
                'e' | 'E' => {
                    if is_exp {
                        panic!("unexpected char found {} expected valid", number);
                    }
                    is_exp = true;
                    number.push(ch);
                    json.next();
                }
                '0'..='9' => {
                    number.push(ch);
                    json.next();
                }
                _ => {
                    if is_float {
                        return JsonDtype::Number(Num::Float(number.parse::<f64>().unwrap()));
                    }
                    return JsonDtype::Number(Num::Integer(number.parse::<i128>().unwrap()));
                }
            }
        }
        panic!("unexpected char found 'EOF' expected '0'..='9'");
    }

    fn parse_boolean(json: &mut Peekable<std::str::Chars>) -> JsonDtype {
        let mut boolean = String::new();
        while let Some(&ch) = json.peek() {
            match ch {
                't' | 'r' | 'u' | 'e' | 'f' | 'a' | 'l' | 's' => {
                    boolean.push(ch);
                    json.next();
                }
                _ => {
                    if boolean == "true" {
                        return JsonDtype::Boolean(true);
                    } else if boolean == "false" {
                        return JsonDtype::Boolean(false);
                    } else {
                        panic!(
                            "unexpected char found {} expected 'true' or 'false'",
                            boolean
                        );
                    }
                }
            }
        }
        panic!("unexpected char found 'EOF' expected 'true' or 'false'");
    }

    fn parse_null(json: &mut Peekable<std::str::Chars>) -> JsonDtype {
        let mut null = String::new();
        while let Some(&ch) = json.peek() {
            match ch {
                'n' | 'u' | 'l' => {
                    null.push(ch);
                    json.next();
                }
                _ => {
                    if null == "null" {
                        return JsonDtype::Null;
                    } else {
                        panic!("unexpected char found {} expected 'null'", null);
                    }
                }
            }
        }
        panic!("unexpected char found 'EOF' expected 'null'");
    }

    fn parse_array(json: &mut Peekable<std::str::Chars>) -> JsonDtype {
        let mut array = Vec::new();
        json.next();
        while json.peek().is_some() {
            skip_whitespaces(json);
            match json.peek().unwrap() {
                ']' => {
                    json.next();
                    return JsonDtype::Array(array);
                }
                ',' => {
                    json.next();
                }
                _ => {
                    array.push(Json::parse_value(json));
                }
            }
        }
        panic!("unexpected char expected ']' found 'EOF'");
    }

    fn parse_object(json: &mut Peekable<std::str::Chars>) -> Json {
        let mut object = Json::new();
        json.next();
        while json.peek().is_some() {
            skip_whitespaces(json);
            if json.peek() == Some(&'}') {
                json.next();
                return object;
            }

            let key = Json::parse_value(json);

            skip_whitespaces(json);
            match json.peek() {
                Some(&':') => {
                    json.next();
                }
                _ => {
                    panic!(
                        "unexpected char expected ':' found {}",
                        json.peek().unwrap()
                    );
                }
            }

            let value = Json::parse_value(json);
            object.insert(key, value);

            skip_whitespaces(json);
            match json.peek() {
                Some(&',') => {
                    json.next();
                }
                Some(&'}') => {
                    json.next();
                    return object;
                }
                _ => {
                    panic!(
                        "unexpected char Expected ',' or '{}' found {}",
                        "}",
                        json.peek().unwrap()
                    );
                }
            }
        }
        panic!("{}", "unexpected char expected '}'");
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    pub fn contains_key(&self, key: &JsonDtype) -> bool {
        self.map.contains_key(key)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<JsonDtype, JsonDtype> {
        self.map.iter()
    }

    pub fn update(&mut self, other: Json) {
        for (key, value) in other.iter() {
            self.map.insert(key.clone(), value.clone());
        }
    }

    pub fn load(file: &mut File) -> Json {
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("read failed");
        Json::parse(&contents)
    }

    pub fn dump(&self, file: &mut File) {
        file.write(self.to_string().as_bytes())
            .expect("write failed");
    }

    pub fn dumps(&self, file: &mut File) {
        file.write(self.stringify().as_bytes())
            .expect("write failed");
    }

    pub fn dumps_pretty(&self, file: &mut File) {
        file.write(self.stringify_pretty().as_bytes())
            .expect("write failed");
    }
}

impl<K> Index<K> for Json
where
    K: Into<JsonDtype>,
{
    type Output = JsonDtype;

    fn index(&self, index: K) -> &Self::Output {
        &self.map.index(&index.into())
    }
}

impl<K> IndexMut<K> for Json
where
    K: Into<JsonDtype>,
{
    fn index_mut(&mut self, index: K) -> &mut JsonDtype {
        self.map.get_mut(&index.into()).unwrap()
    }
}

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{")?;
        for (i, (key, value)) in self.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
        }
        write!(f, "}}")
    }
}

fn main() {
    let mut json_obj = Json::parse(
        r#"{"Hello": "World!", "potatoes": [1, 2, 3, { "a": 1 , "b": false, "c": null }],}"#,
    );
    println!("{}", json_obj);

    json_obj.insert("age", 20);
    println!("{}", json_obj.stringify_pretty());

    println!("{}", json_obj.get("Hello").unwrap());

    json_obj.remove("Hello");
    println!("{}", json_obj);

    let mut json_obj2 = Json::new();
    json_obj2.insert("age", 21);

    println!("{}", json_obj2["age"]);
    json_obj2["age"] = 22.into();
    println!("{}", json_obj2["age"]);

    json_obj.update(json_obj2);
    println!("{}", json_obj);


    let mut data_file = File::create("data.json").expect("creation failed");
    json_obj.dumps_pretty(&mut data_file);
}
