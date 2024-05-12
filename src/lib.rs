#![deny(clippy::all)]

use std::collections::HashMap;

#[macro_use]
extern crate napi_derive;

/// Checks a text for slurs and determines the level of profanity of this, levels go from 0 to 3
#[napi]
pub fn check_text(text: napi::JsString, max_level: Option<i32>) -> bool {
    let slurs: HashMap<&str, i32> = HashMap::from([
      ("cunt", 1),
      ("faggot", 3)
    ]);

    let binding = text.into_utf8().unwrap();
    let text_str = binding.as_str().unwrap();
    let words = text_str.split(" ");

    for word in words {
      if slurs.contains_key(word) && slurs.get(word) == Some(max_level.as_ref().unwrap_or(&1)) {
        return true
      }
    }
    
    return false
}
