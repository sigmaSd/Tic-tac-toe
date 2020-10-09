fn main() {
    let mut b: Board = Board {
        b: [None, None, None, None, None, None, None, None, None],
        rules: WinRule::new(),
    };
    loop {
        b.render_board();
        b.player_turn();
        b.check_for_end();
        b.bot_turn();
        b.check_for_end();
    }
}

#[derive(PartialEq, Copy, Clone)]
enum C {
    X,
    O,
    XWin,
    OWin,
}
struct Board {
    b: [Option<C>; 9],
    rules: WinRule,
}

struct WinRule {
    a: Box<dyn Fn(&[Option<C>], usize) -> bool>,
    b: Box<dyn Fn(&[Option<C>], usize) -> bool>,
    c: Box<dyn Fn(&[Option<C>], usize) -> bool>,
    d: Box<dyn Fn(&[Option<C>], usize) -> bool>,
}

impl WinRule {
    fn new() -> Self {
        Self {
            a: Box::new(|b, c| c % 3 == 0 && b[c] == b[c + 1] && b[c] == b[c + 2]),
            b: Box::new(|b, c| b[c] == b[(c + 3) % 9] && b[c] == b[(c + 6) % 9]),
            c: Box::new(|b, c| c == 0 && b[c] == b[c + 4] && b[c] == b[c + 8]),
            d: Box::new(|b, c| c == 2 && b[c] == b[c + 2] && b[c] == b[c + 4]),
        }
    }
    fn matches(&self, b: &[Option<C>], c: usize) -> Option<WinLayout> {
        // return early if b[c] is none
        b[c].as_ref()?;

        use WinLayout::*;
        if (self.a)(b, c) {
            Some(A)
        } else if (self.b)(b, c) {
            Some(B)
        } else if (self.c)(b, c) {
            Some(C)
        } else if (self.d)(b, c) {
            Some(D)
        } else {
            None
        }
    }
}

enum WinLayout {
    A, // ---
    B, // |
    C, // \
    D, // /
}

impl Board {
    fn check_for_end(&mut self) {
        // 0 1 2
        // 3 4
        // 6   8
        for c in 0..self.b.len() {
            if let Some(wl) = self.rules.matches(&self.b, c) {
                let msg = match self.b[c] {
                    Some(C::X) => "Human wins",
                    Some(C::O) => "Bot wins",
                    _ => unreachable!(),
                };
                self.render_win_board(wl, c);

                println!("{}", msg);
                println!();
                std::process::exit(0);
            }
        }
    }

    fn render_win_board(&mut self, wl: WinLayout, c: usize) {
        let w = match self.b[c].unwrap() {
            crate::C::X => Some(crate::C::XWin),
            crate::C::O => Some(crate::C::OWin),
            _ => unreachable!(),
        };
        use WinLayout::*;
        match wl {
            A => {
                self.b[c] = w;
                self.b[c + 1] = w;
                self.b[c + 2] = w;
            }
            B => {
                self.b[c] = w;
                self.b[(c + 3) % 9] = w;
                self.b[(c + 6) % 9] = w;
            }
            C => {
                self.b[c] = w;
                self.b[c + 4] = w;
                self.b[c + 8] = w;
            }
            D => {
                self.b[c] = w;
                self.b[c + 2] = w;
                self.b[c + 4] = w;
            }
        }
        self.render_board();
    }

    fn render_board(&self) {
        println!();
        for (i, c) in self.b.iter().enumerate() {
            if i % 3 == 0 {
                println!();
            }
            match c {
                None => print!("_"),
                Some(C::X) => print!("X"),
                Some(C::O) => print!("O"),
                Some(C::OWin) => print!("𝗢"),
                Some(C::XWin) => print!("🗴"),
            }
        }
        println!();
    }
    fn player_turn(&mut self) {
        while self.try_play().is_none() {
            eprintln!("Invalid move");
            self.render_board();
        }
    }

    fn bot_turn(&mut self) {
        use std::time::{SystemTime, UNIX_EPOCH};
        let available = self.b.iter().filter(|c| c.is_none()).count();
        let rnd = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as usize
            % available;
        *self.b.iter_mut().filter(|c| c.is_none()).nth(rnd).unwrap() = Some(C::O);
    }

    fn try_play(&mut self) -> Option<()> {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok()?;

        let mut input = input.chars();
        let n = input.next()?.to_digit(10)?;
        let c = input.next()?;

        let choice = match c {
            'x' | 'X' => C::X,
            //'o' | 'O' => C::O,
            _ => return None,
        };
        self.set(n, choice)?;
        Some(())
    }

    fn set(&mut self, n: u32, choice: C) -> Option<()> {
        let n = match n {
            1 => 6,
            2 => 7,
            3 => 8,
            4 => 3,
            5 => 4,
            6 => 5,
            7 => 0,
            8 => 1,
            9 => 2,
            _ => {
                eprintln!("Invalid position");
                return None;
            }
        };
        if self.b[n].is_some() {
            eprintln!("Ocuupied cell!");
            return None;
        }
        self.b[n] = Some(choice);
        Some(())
    }
}
