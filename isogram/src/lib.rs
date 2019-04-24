use std::collections::HashSet; //reference -> https://doc.rust-lang.org/std/collections/struct.HashSet.html

pub fn check(candidate: &str) -> bool {
    let mut isog = HashSet::new();
    let mut ignore = 0;

    for i in candidate.to_lowercase().chars(){
        if i.is_alphanumeric(){ 
            isog.insert(i);
        }else{
            ignore += 1;
        }
    }
    candidate.len() == isog.len() + ignore

}

