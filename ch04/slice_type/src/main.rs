fn main() {
    let mut s = String::from("hello world");
    let word = first_word_ending_index(&s);
    let first_word1 = first_word(&s);

    // what if we modified if halfway?
    let s = "bye"; // this will compile but generate a panic at runtime
                   // println!("first word is: {}", &s[..word]); // this is a slice of the string
    let first_word2 = first_word(&s);
    println!("first word (using copy): {}", first_word1);
    println!("first word again: {}", first_word2);

    // array kinds of slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]); // assertion will pass
}

fn first_word_ending_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word(s: &str) -> String {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[0..i]).to_string();
        }
    }
    return s.to_string();
}
