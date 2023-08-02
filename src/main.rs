use rand::{seq::SliceRandom, Rng};
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

fn main() {
    let _stdout = std::io::stdout().into_raw_mode().unwrap();
    let terminal_size = termion::terminal_size().unwrap();

    print!(
        "{}{}{}{}",
        termion::clear::All,
        termion::color::Fg(COLOR),
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    );

    if let Err(e) = refresh_screen(terminal_size) {
        panic!("{}", e);
    }
}

fn refresh_screen(terminal_size: (u16, u16)) -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let x_pos = rng.gen_range(0..terminal_size.0) + 1;
    let character = CHARACTERS.choose(&mut rng).unwrap();

    print!(
        "{}{}{}",
        termion::color::Fg(COLOR),
        termion::cursor::Goto(x_pos, 1),
        character
    );

    Ok(())
}
