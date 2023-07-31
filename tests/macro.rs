use easy_sgr::sgr;

macro_rules! sgr_tests {
    ($($input:tt = $result:literal),*) => {
        $(
            assert_eq!($result, sgr!($input));
        )*
    };
}

#[test]
fn styles() {
    sgr_tests!(
        "{+Reset}" = "\x1b[0m",
        "{+Bold}" = "\x1b[1m",
        "{+Dim}" = "\x1b[2m",
        "{+Italic}" = "\x1b[3m",
        "{+Underline}" = "\x1b[4m",
        "{+Blink}" = "\x1b[5m",
        "{+Inverse}" = "\x1b[7m",
        "{+Hide}" = "\x1b[8m",
        "{+Strike}" = "\x1b[9m",
        "{-Bold}" = "\x1b[22m",
        "{-Dim}" = "\x1b[22m",
        "{-Italic}" = "\x1b[23m",
        "{-Underline}" = "\x1b[24m",
        "{-Blink}" = "\x1b[25m",
        "{-Inverse}" = "\x1b[27m",
        "{-Hide}" = "\x1b[28m",
        "{-Strike}" = "\x1b[29m"
    );
}
#[test]
fn standard_colors() {
    sgr_tests!(
        "{#BlackFg}" = "\x1b[30m",
        "{#RedFg}" = "\x1b[31m",
        "{#GreenFg}" = "\x1b[32m",
        "{#YellowFg}" = "\x1b[33m",
        "{#BlueFg}" = "\x1b[34m",
        "{#MagentaFg}" = "\x1b[35m",
        "{#CyanFg}" = "\x1b[36m",
        "{#WhiteFg}" = "\x1b[37m",
        "{#DefaultFg}" = "\x1b[39m",
        "{#BlackBg}" = "\x1b[40m",
        "{#RedBg}" = "\x1b[41m",
        "{#GreenBg}" = "\x1b[42m",
        "{#YellowBg}" = "\x1b[43m",
        "{#BlueBg}" = "\x1b[44m",
        "{#MagentaBg}" = "\x1b[45m",
        "{#CyanBg}" = "\x1b[46m",
        "{#WhiteBg}" = "\x1b[47m",
        "{#DefaultBg}" = "\x1b[49m"
    );
}

#[test]
fn byte_color() {
    sgr_tests!(
        "{#f(0)}" = "\x1b[38;5;0m",
        "{#b(0)}" = "\x1b[48;5;0m",
        "{#f(255)}" = "\x1b[38;5;255m",
        "{#b(255)}" = "\x1b[48;5;255m",
        "{#f[00]}" = "\x1b[38;5;0m",
        "{#b[00]}" = "\x1b[48;5;0m",
        "{#f[ff]}" = "\x1b[38;5;255m",
        "{#b[ff]}" = "\x1b[48;5;255m"
    );
}

#[test]
fn rgb_color() {
    sgr_tests!(
        "{#f(0,0,0)}" = "\x1b[38;2;0;0;0m",
        "{#b(0,0,0)}" = "\x1b[48;2;0;0;0m",
        "{#f(255,255,255)}" = "\x1b[38;2;255;255;255m",
        "{#b(255,255,255)}" = "\x1b[48;2;255;255;255m",
        "{#f[000000]}" = "\x1b[38;2;0;0;0m",
        "{#b[000000]}" = "\x1b[48;2;0;0;0m",
        "{#f[ffffff]}" = "\x1b[38;2;255;255;255m",
        "{#b[ffffff]}" = "\x1b[48;2;255;255;255m"
    );
}
