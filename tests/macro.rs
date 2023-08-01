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
        "{[reset]}" = "\x1b[0m",
        "{[bold]}" = "\x1b[1m",
        "{[dim]}" = "\x1b[2m",
        "{[italic]}" = "\x1b[3m",
        "{[underline]}" = "\x1b[4m",
        "{[blink]}" = "\x1b[5m",
        "{[inverse]}" = "\x1b[7m",
        "{[hide]}" = "\x1b[8m",
        "{[strike]}" = "\x1b[9m",
        "{[!bold]}" = "\x1b[22m",
        "{[!dim]}" = "\x1b[22m",
        "{[!italic]}" = "\x1b[23m",
        "{[!underline]}" = "\x1b[24m",
        "{[!blink]}" = "\x1b[25m",
        "{[!inverse]}" = "\x1b[27m",
        "{[!hide]}" = "\x1b[28m",
        "{[!strike]}" = "\x1b[29m"
    );
}
#[test]
fn standard_colors() {
    sgr_tests!(
        "{[black]}" = "\x1b[30m",
        "{[red]}" = "\x1b[31m",
        "{[green]}" = "\x1b[32m",
        "{[yellow]}" = "\x1b[33m",
        "{[blue]}" = "\x1b[34m",
        "{[magenta]}" = "\x1b[35m",
        "{[cyan]}" = "\x1b[36m",
        "{[white]}" = "\x1b[37m",
        "{[default]}" = "\x1b[39m",
        "{[on-black]}" = "\x1b[40m",
        "{[on-red]}" = "\x1b[41m",
        "{[on-green]}" = "\x1b[42m",
        "{[on-yellow]}" = "\x1b[43m",
        "{[on-blue]}" = "\x1b[44m",
        "{[on-magenta]}" = "\x1b[45m",
        "{[on-cyan]}" = "\x1b[46m",
        "{[on-white]}" = "\x1b[47m",
        "{[on-default]}" = "\x1b[49m"
    );
}

#[test]
fn byte_color() {
    sgr_tests!(
        "{[0]}" = "\x1b[38;5;0m",
        "{[on-0]}" = "\x1b[48;5;0m",
        "{[255]}" = "\x1b[38;5;255m",
        "{[on-255]}" = "\x1b[48;5;255m",
        "{[#00]}" = "\x1b[38;5;0m",
        "{[on-#00]}" = "\x1b[48;5;0m",
        "{[#ff]}" = "\x1b[38;5;255m",
        "{[on-#ff]}" = "\x1b[48;5;255m"
    );
}

#[test]
fn rgb_color() {
    sgr_tests!(
        "{[0,0,0]}" = "\x1b[38;2;0;0;0m",
        "{[on-0,0,0]}" = "\x1b[48;2;0;0;0m",
        "{[255,255,255]}" = "\x1b[38;2;255;255;255m",
        "{[on-255,255,255]}" = "\x1b[48;2;255;255;255m",
        "{[#000000]}" = "\x1b[38;2;0;0;0m",
        "{[on-#000000]}" = "\x1b[48;2;0;0;0m",
        "{[#ffffff]}" = "\x1b[38;2;255;255;255m",
        "{[on-#ffffff]}" = "\x1b[48;2;255;255;255m"
    );
}