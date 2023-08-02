use rand::seq::SliceRandom;
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

    print!(
        "{}{}{}{}",
        termion::clear::All,
        termion::color::Fg(COLOR),
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide
    );

    if let Err(e) = refresh_screen() {
        panic!("{}", e);
    }
}

fn refresh_screen() -> Result<(), std::io::Error> {
    let character = CHARACTERS.choose(&mut rand::thread_rng()).unwrap();

    print!("Hello 1\n");
    print!("{}Hello 2\n", termion::cursor::Goto(1, 2));
    print!("{}Hello 3\n", termion::cursor::Goto(1, 1));

    print!("{}{}", termion::cursor::Goto(1, 3), character);

    Ok(())
}
