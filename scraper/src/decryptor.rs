use regex::Regex;

pub struct JavascriptDecryptor {
    
    decryptor_regexes: Vec<Regex>
}

impl JavascriptDecryptor {

    pub fn new() -> Self {
        
        let regexes = vec![

            Regex::new("").unwrap(),
            Regex::new("").unwrap()
        ];

        return Self { decryptor_regexes: regexes };
    }

    pub fn find_function_signature(&self, js: &str) -> Option<String> {

        let try_decryptor = | decryptor: &Regex | map_match_to_str(decryptor.find(js));
        return self.decryptor_regexes.as_slice().into_iter().find_map(try_decryptor);
    }
}

fn map_match_to_str<'h>(haystack: Option<regex::Match<'h>>) -> Option<String> {

    if let None = haystack {
        return None;
    }

    return Some(haystack.unwrap().as_str().to_owned());
}  

