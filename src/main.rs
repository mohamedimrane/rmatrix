use rand::{seq::SliceRandom, Rng};
use std::io::{Stdout, Write};
use termion::{
    cursor::DetectCursorPos,
    raw::{IntoRawMode, RawTerminal},
};

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
    length: u16,
    x_pos: u16,
    y_offset: u16,
    speed: std::time::Duration,
    next_time: std::time::Instant,
    characters: Vec<char>,
}

impl Drop {
    fn draw_if_timer_is_elapsed(&mut self, stdout: &mut RawTerminal<Stdout>) {
        if std::time::Instant::now() >= self.next_time {
            print!("{}", termion::cursor::Goto(self.x_pos, self.y_offset));
            let (x, y) = stdout.cursor_pos().unwrap();
            write!(stdout, "{}{}", " ", termion::cursor::Goto(x - 1, y));

            for (index, c) in self.characters.iter().enumerate() {
                let iu16 = index as u16;
                if iu16 < self.y_offset || iu16 > self.length + self.y_offset {
                    continue;
                }

                let offset = self.y_offset as usize + 1;
                draw(*c, (self.x_pos, (index + offset) as u16), COLOR);
            }

            self.y_offset += 1;

            self.reset_timer();
        }
    }

    fn reset_timer(&mut self) {
        self.next_time += self.speed;
    }
}

fn main() {
    let mut stdout = std::io::stdout().into_raw_mode().unwrap();
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
        if let Err(e) = refresh_screen(&mut drops, terminal_size, &mut stdout, &mut next_time_spawn)
        {
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
    stdout: &mut RawTerminal<Stdout>,
    next_time_spawn: &mut std::time::Instant,
) -> Result<(), std::io::Error> {
    if std::time::Instant::now() >= *next_time_spawn {
        let mut rng = rand::thread_rng();
        let length = rng.gen_range(5..=20);
        let x_pos = rng.gen_range(0..terminal_size.0) + 1;
        let speed = std::time::Duration::from_millis(rng.gen_range(100..=200));
        let characters = generate_character_vec(terminal_size.1, &mut rng);

        drops.push(Drop {
            length,
            x_pos,
            y_offset: 0,
            speed,
            next_time: std::time::Instant::now(),
            characters,
        });

        *next_time_spawn += std::time::Duration::from_millis(100);
    }

    let mut index = 0;
    while index < drops.len() {
        drops[index].draw_if_timer_is_elapsed(stdout);
        if drops[index].y_offset > terminal_size.1 {
            drops.remove(index);
            continue;
        }
        index += 1;
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
