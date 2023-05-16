# rust-json

##### Implementation of json in rust using enums and structs

### Basic Methods
```rust
Json::new(); // makes new empty json
json_obj.insert(key, value); // adds key value pair to json
// allowed data types are i128, f64, bool, () for null, string, JsonDtype, Json
json_obj.get(<key>); // returns value corresponding to the key -> Option<&JsonDtype>
json_obj.remove(<key>); // removes corresponding value
json_obj.update(json_obj2); // updates values 
json_obj.contains_key(<key>);
json_obj.len();
json_obj.is_empty();
json_obj.clear();
```

### Looping and iteration
```rust
json_obj.iter(); // return iterable
json_obj.keys(); // return all keys
```

### JsonDtypes
```rust
enum JsonDtype {
    String(String)  // String
    Number(Num)  // i128 or f64
    Object(Json)  // Json object
    Array(Vec<JsonDtype>) // vector of JsonDtype
    Boolean(bool) // true/false
    Null
}
```

### Coversion Json/String
```rust
Json::parse("{"name": "nikhil"}"); // parses json from string
json_obj.stringify(); // return stringified json
json_obj.stringify_pretty(); // returns decorated json (with indentation (4sapces))
```

### save/load to/from file
```rust
Json.load(&mut std::fs::File); // loads json from file
json_obj.dump(&mut std::fs::File); // writes raw json to file
json_obj.dumps(&mut std::fs::File); // writes stringified json to file
json_obj.dumps_pretty(&mut std::fs::File); // writes pretty json to file
```

### Example 
```rust
mod json;

json::Json;

fn main() {
    let mut json_obj = Json::parse(r#"{"Hello": "World!", "potatoes": [1, 2, 3, { "a": 1 , "b": false, "c": null }],}"#);
    println!("{}", json_obj);
    
    json_obj.insert("age", 20);
    println!("{}", json_obj.stringify_pretty());
    
    println!("{}", json_obj.get("Hello").unwrap());
    
    json_obj.remove("Hello");
    println!("{}", json_obj);
    
    let mut json_obj2 = Json::new();
    json_obj2.insert("age", 21);

    json_obj.update(json_obj2);
    println!("{}", json_obj);

    let mut data_file = File::create("data.json").expect("creation failed");

    json_obj.dumps_pretty(&mut data_file);
}
```


### OUTPUT
```json
{"potatoes": [1, 2, 3, {"b": false, "c": null, "a": 1}], "Hello": "World!"}
{
    "age": 20,
    "potatoes": [
        1,
        2,
        3,
        {
            "b": false,
            "c": null,
            "a": 1
        }
    ],
    "Hello": "World!"
}
"World!"
{"age": 20, "potatoes": [1, 2, 3, {"b": false, "c": null, "a": 1}]}
{"age": 21, "potatoes": [1, 2, 3, {"b": false, "c": null, "a": 1}]}
```


### Data.json
```json
{
    "age": 21,
    "potatoes": [
        1,
        2,
        3,
        {
            "b": false,
            "c": null,
            "a": 1
        }
    ]
}
```
