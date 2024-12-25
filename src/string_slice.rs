pub fn split_word(sentence: &String) -> &str{

    let bytes = sentence.as_bytes();

    for (i, &character) in bytes.iter().enumerate(){
        if character == b' '{
            return &sentence[0..i];
        }
    }

    return &sentence[..];
}