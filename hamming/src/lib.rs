/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    //unimplemented!("What is the Hamming Distance between {} and {}", s1, s2);

    let chara_s1: Vec<char> = s1.chars().collect();
    let chara_s2: Vec<char> = s2.chars().collect();
    let mut counter = 0;

    if s1.len() != s2.len() {
        None
    }else{
         for i in 0..s1.len(){
            if chara_s1[i] != chara_s2[i] {
                counter += 1;
            }
        }
        Some(counter)
    }
}
