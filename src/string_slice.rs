pub fn _split_word(sentence: &String) -> &str{

    let bytes = sentence.as_bytes();

    for (i, &character) in bytes.iter().enumerate(){
        if character == b' '{
            return &sentence[0..i];
        }
    }

    return &sentence[..];
}

pub fn _split_word_alt(sentence: &String) -> &str{

    for (index, character) in sentence.chars().enumerate(){
        if character == ' '{
            return &sentence[0..index];
        }
    }

    return &sentence[..];
}