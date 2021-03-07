use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Easy Pass")]
pub struct Config {
    /// If enabled similar looking characters or symbols will be used
    #[structopt(short = "s", long)]
    substitute: bool,

    /// Inclusion of special characters like: !@#$%^&*()_+=[]{};':"\|<>,.?/
    #[structopt(short = "S", long)]
    include_special: bool,

    /// Random hex value
    #[structopt(short = "h", long)]
    hex_value: bool,

    /// Length of password
    #[structopt(short = "l", long, default_value = "20")]
    password_length: u8,

    /// Chance of number in password
    #[structopt(short = "c", long, default_value = "30")]
    number_chance: u8,

    /// Words to use in password
    words: Vec<String>,
}

impl Config {
    pub fn new(
        substitute: bool,
        include_special: bool,
        hex_value: bool,
        password_length: u8,
        number_chance: u8,
        words: Vec<String>,
    ) -> Self {
        Self {
            substitute,
            include_special,
            hex_value,
            password_length,
            number_chance,
            words,
        }
    }

    pub fn substitute(&self) -> bool {
        self.substitute
    }

    pub fn include_special(&self) -> bool {
        self.include_special
    }

    pub fn hex_value(&self) -> bool {
        self.hex_value
    }

    pub fn password_length(&self) -> u8 {
        self.password_length
    }

    pub fn number_chance(&self) -> u8 {
        self.number_chance
    }

    pub fn words(&self) -> &Vec<String> {
        &self.words
    }
}
