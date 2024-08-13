use serde::*;

// std::result::Result<String, serde_json::Error>
pub fn encode_gen<T>(v: &T) -> std::io::Result<String>
    where T: ?Sized + Serialize 
    {
        #[allow(non_snake_case)]
        let maybeString = serde_json::to_string(v);
        match maybeString {
            Err (e) => {panic!("Error Encoding val: {e:?}")}
            Ok (s) => Ok (s)
        }
    }

// std::result::Result<T, serde_json::Error> 
pub fn decode_gen<'a, T>(s: &'a str) -> std::io::Result<T>
    where T: de::Deserialize<'a>
    {
        #[allow(non_snake_case)]
        let maybeVal = serde_json::from_str(s);
        match maybeVal {
            Err (e) => {panic!("Error Decoding val: {e:?}")}
            Ok (v) => Ok (v)
        }  
    }