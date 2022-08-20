const SUFFIX: &str = "ay";
const VOWEL_MOD: char = 'h';

fn main() {
    let words = vec!["pig", "dread", "latin", "banana", "will", "apple", "eat"];

    println!("original words: {:?}", words);
    println!("pig latin: {:?}", pig_latin(&words));
}

fn pig_latin(words: &[&str]) -> Vec<String> {
    let mut new_words: Vec<String> = Vec::new();

    for word in words {
        let mut new_word: String = String::new();

        if is_consonant_first(word) {
            new_word.push_str(&move_consonants(word));
        } else {
            new_word.push_str(word);
            new_word.push(VOWEL_MOD);
        };

        new_word.push_str(SUFFIX);

        new_words.push(new_word);
    }

    new_words
}

fn move_consonants(word: &str) -> String {
    let mut cluster: String = String::new();
    let mut new_word: String = String::new();

    for chr in word.chars() {
        if is_consonant(chr) {
            cluster.push(chr)
        } else {
            break;
        }
    }

    new_word.push_str(word.get(cluster.len()..).unwrap());
    new_word.push_str(&cluster);
    new_word
}

fn is_consonant_first(word: &str) -> bool {
    is_consonant(word.chars().next().unwrap())
}

fn is_consonant(chr: char) -> bool {
    !matches!(chr, 'a' | 'o' | 'e' | 'i' | 'u')
}
