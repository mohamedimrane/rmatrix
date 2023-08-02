use rand::{seq::SliceRandom, Rng};
use std::io::Write;
use termion::raw::IntoRawMode;

const CHARACTERS: &[char] = &[
    'ｦ', 'ｧ', 'ｨ', 'ｩ', 'ｪ', 'ｫ', 'ｬ', 'ｭ', 'ｮ', 'ｯ', 'ｰ', 'ｱ', 'ｲ', 'ｳ', 'ｴ', 'ｵ', 'ｶ', 'ｷ', 'ｸ',
    'ｹ', 'ｺ', 'ｻ', 'ｼ', 'ｽ', 'ｾ', 'ｿ', 'ﾀ', 'ﾁ', 'ﾂ', 'ﾃ', 'ﾄ', 'ﾅ', 'ﾆ', 'ﾇ', 'ﾈ', 'ﾉ', 'ﾊ', 'ﾋ',
    'ﾌ', 'ﾍ', 'ﾎ', 'ﾏ', 'ﾐ', 'ﾑ', 'ﾒ', 'ﾓ', 'ﾔ', 'ﾕ', 'ﾖ', 'ﾗ', 'ﾘ', 'ﾙ', 'ﾚ', 'ﾜ', 'ﾝ', 'A', 'B',
    'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U',
    'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '-', '!', '@',
    '$', '%', '^', '&', '*', '(', ')', '§', '=', '{', '}', '[', ']', '|', '?', '>', '<', '/', '~',
    ';', ':', 'ß', '∂', 'h', '∆', 'ø', 'π', 'å', '≥', '≤', '÷', '¿', '◊', '¡', '€', '#', '∞', '≠',
    '‡', '≈',
];
const COLOR: termion::color::Rgb = termion::color::Rgb(46, 248, 47);

struct Drop {
    lenght: u16,
    x_pos: u16,
    speed: u64,
    characters: Vec<char>,
}

fn main() {
    let _stdout = std::io::stdout().into_raw_mode().unwrap();
    let terminal_size = termion::terminal_size().unwrap();

    print!(
        "{}{}{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    );

    let mut drops = Vec::new();

    let mut next_time_spawn = std::time::Instant::now();
    loop {
        if let Err(e) = refresh_screen(&mut drops, terminal_size, &mut next_time_spawn) {
            panic!("{}", e);
        }

        if let Err(e) = std::io::stdout().flush() {
            panic!("{}", e);
        }
    }
}

fn refresh_screen(
    drops: &mut Vec<Drop>,
    terminal_size: (u16, u16),
    next_time_spawn: &mut std::time::Instant,
) -> Result<(), std::io::Error> {
    if std::time::Instant::now() >= *next_time_spawn {
        let mut rng = rand::thread_rng();
        let x_pos = rng.gen_range(0..terminal_size.0) + 1;
        let speed = rng.gen_range(100..=200);
        let characters = generate_character_vec(terminal_size.1, &mut rng);

        drops.push(Drop {
            lenght: terminal_size.1,
            x_pos,
            speed,
            characters,
        });

        for drop in drops.iter() {
            for (index, c) in drop.characters.iter().enumerate() {
                draw(*c, (drop.x_pos, (index + 1) as u16), COLOR);
            }
        }

        *next_time_spawn += std::time::Duration::from_millis(200);
    }

    Ok(())
}

fn draw(character: char, pos: (u16, u16), color: termion::color::Rgb) {
    print!(
        "{}{}{}{}",
        termion::cursor::Goto(pos.0, pos.1),
        termion::color::Fg(color),
        character,
        termion::color::Fg(termion::color::Reset)
    );
}

fn generate_character_vec(length: u16, rng: &mut rand::rngs::ThreadRng) -> Vec<char> {
    let mut result = Vec::with_capacity(length as usize);

    for _ in 0..length {
        let character = CHARACTERS.choose(rng).unwrap();
        result.push(*character);
    }

    result
}
