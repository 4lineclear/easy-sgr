#[cfg(feature = "macros")]
mod macros {
    use std::fmt::Write;

    use easy_sgr::{eprint, eprintln, format, format_args, print, println, sgr, write, writeln};

    macro_rules! sgr_tests {
        ($($input:tt = $result:literal),*) => {
            $(
                assert_eq!($result, sgr!($input));
            )*
        };
    }
    #[test]
    fn general() {
        sgr_tests!(
            "" = "",
            "{[]}" = "\x1b[0m",
            "{[reset bold]}" = "\x1b[0;1m",
            "{[!bold]}" = "\x1b[22m",
            "{[0,0,0 on-0,0,0]}" = "\x1b[38;2;0;0;0;48;2;0;0;0m",
            "{[#00 on-#00]}" = "\x1b[38;5;0;48;5;0m",
            "{{[]}}" = "{{[]}}"
        );
    }
    #[test]
    fn raw_strings() {
        sgr_tests!(
            r"Not much to test for this one maybe, this can't really fail" =
                "Not much to test for this one maybe, this can't really fail"
        );
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
    /// really just for improving coverage numbers
    /// the other tests cover pretty much everything
    #[test]
    fn etc() {
        let formatted = format!("Normal{[green]} now this is green{[]} and this is not");
        let from_args = std::fmt::format(format_args!(
            "Normal{[green]} now this is green{[]} and this is not"
        ));
        let mut written_to = String::new();

        writeln!(written_to).unwrap();
        write!(
            written_to,
            "Normal{[green]} now this is green{[]} and this is not"
        )
        .unwrap();
        writeln!(written_to).unwrap();

        println!();
        print!("Normal{[green]} now this is green{[]} and this is not");
        println!();

        eprintln!();
        eprint!("Normal{[green]} now this is green{[]} and this is not");
        eprintln!();

        assert_eq!(
            formatted,
            "Normal\u{1b}[32m now this is green\u{1b}[0m and this is not"
        );
        assert_eq!(
            from_args,
            "Normal\u{1b}[32m now this is green\u{1b}[0m and this is not"
        );
        assert_eq!(
            written_to,
            "\nNormal\u{1b}[32m now this is green\u{1b}[0m and this is not\n"
        );
    }
}
