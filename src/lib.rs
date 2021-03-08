//! Every optimization is taken to make passwords or hex numbers to as fast as possible
mod config;

use config::Config;
use rand::{self, Rng};
use structopt::StructOpt;

const VALID_CHARS: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const SPECIAL_CHARS: [char; 28] = [
    '~', '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '=', '+', '[', ']', '{',
    '}', ';', ':', '\'', '"', '\\', '|', '/', '?',
];

pub fn run() {
    let config = Config::from_args();
    // println!("Current {:#?}", config);

    let password = if config.hex_value() {
        generate_hex_number(&config)
    } else {
        let mut allowed_chars = VALID_CHARS.to_vec();

        if config.include_special() {
            allowed_chars.append(&mut SPECIAL_CHARS.to_vec());
        }

        if let Some(words) = string_from_words(config.words()) {
            password_with_words(words, &config, &allowed_chars)
        } else {
            generate_password(&config, &allowed_chars, config.password_length())
        }
    };

    println!("{}", password);
}

fn password_with_words(word: String, config: &Config, allowed_chars: &Vec<char>) -> String {
    if word.len() < config.password_length() {
        let random_pass = generate_password(
            &config,
            &allowed_chars,
            config.password_length() - word.len(),
        );

        let new_word = if config.substitute() {
            modify_word(word)
        } else {
            word
        };

        format!("{}{}", new_word, random_pass)
    } else {
        word
    }
}

fn string_from_words(words: &Vec<String>) -> Option<String> {
    if words.len() > 0 {
        let mut result = String::new();

        for word in words {
            result.push_str(&word);
        }

        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    } else {
        None
    }
}

fn generate_password(config: &Config, chars: &[char], length: usize) -> String {
    let mut result = String::with_capacity(length);
    let chars_len = chars.len();
    let mut rng = rand::thread_rng();

    while result.len() < length {
        if config.number_chance() < rng.gen_range(0..100) {
            let index = rng.gen_range(0..chars_len);
            let mut ch = chars[index];

            if config.substitute() {
                ch = substitute_char(ch);
            }

            if rand::random() {
                result.push(ch)
            } else {
                result.push(ch.to_lowercase().next().unwrap());
            }
        } else {
            let number = rng.gen_range(0..10);
            result.push_str(number.to_string().as_str());
        }
    }

    result
}

fn generate_hex_number(config: &Config) -> String {
    let length = config.password_length() as usize;
    let mut result = String::with_capacity(length);
    let mut rng = rand::thread_rng();

    while result.len() < length {
        let number = rng.gen_range(0..16);
        result.push_str(format!("{:x}", number).as_str());
    }

    result
}

fn modify_word(word: String) -> String {
    word.chars().map(|ch| substitute_char(ch)).collect()
}

fn substitute_char(ch: char) -> char {
    match ch.to_uppercase().next().unwrap_or(ch) {
        '0' => 'O',
        '1' => 'L',
        '2' => 'Z',
        '3' => 'E',
        '4' => 'A',
        '5' => 'S',
        '6' => 'B',
        '7' => 'L',
        '8' => 'B',
        '9' => 'G',
        'A' => '4',
        'B' => '8',
        'C' => 'U',
        'D' => '0',
        'E' => '3',
        //'F' => 'X',
        'G' => '6',
        //'H' => 'X',
        'I' => '1',
        //'J' => 'X',
        //'K' => 'X',
        'L' => '1',
        //'M' => 'X',
        //'N' => 'X',
        'O' => '0',
        //'P' => 'X',
        //'Q' => 'X',
        //'R' => 'X',
        'S' => '5',
        //'T' => 'X',
        //'U' => 'X',
        //'V' => 'X',
        //'W' => 'X',
        //'X' => 'X',
        //'Y' => 'X',
        'Z' => '2',
        _ => ch,
    }
}

//fn words_total_length(words: &Vec<String>) -> usize {
//    words.iter().map(|word| word.len()).sum()
//}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::new(
            false,
            false,
            false,
            20,
            30,
            vec!["one".to_string(), "two".to_string(), "three".to_string()],
        )
    }
    #[test]
    fn correct_length() {
        let config = test_config();
        let password = generate_random_password(&config, &VALID_CHARS);
        println!("Generated password: {}", password);
        assert_eq!(password.len(), config.password_length() as usize);
    }

    #[test]
    fn hex_number() {
        let config = test_config();
        let number = generate_hex_number(&config);
        assert_eq!(number.len(), config.password_length() as usize);
    }

    #[test]
    fn substitution() {
        assert_eq!(substitute_char('E'), '3');
        assert_eq!(substitute_char('1'), 'L');
        assert_eq!(substitute_char('x'), 'x');
    }

    #[test]
    fn total_length_of_words() {
        let config = test_config();
        assert_eq!(words_total_length(&config.words()), 11);
    }
}
