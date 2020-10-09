use std::io::BufRead;

fn main() {
    let mut b: Board = Board {
        b: [None, None, None, None, None, None, None, None, None],
        rules: WinRule::new(),
        input: String::new(),
    };
    loop {
        b.render_board();
        b.player_turn();
        b.check_for_end();
        b.bot_turn();
        b.check_for_end();
    }
}

#[derive(Copy, Clone)]
enum C {
    X,
    O,
    XWin,
    OWin,
}

impl PartialEq for C {
    fn eq(&self, other: &Self) -> bool {
        use C::*;
        match (self, other) {
            (X, X) | (X, XWin) | (XWin, X) => true,
            (O, O) | (O, OWin) | (OWin, O) => true,
            _ => false,
        }
    }
}

struct Board {
    b: [Option<C>; 9],
    rules: WinRule,
    input: String,
}

struct WinRule {
    a: fn(&[Option<C>], usize) -> bool,
    b: fn(&[Option<C>], usize) -> bool,
    c: fn(&[Option<C>], usize) -> bool,
    d: fn(&[Option<C>], usize) -> bool,
}

impl WinRule {
    fn new() -> Self {
        Self {
            a: |b, c| c % 3 == 0 && b[c] == b[c + 1] && b[c] == b[c + 2],
            b: |b, c| b[c] == b[(c + 3) % 9] && b[c] == b[(c + 6) % 9],
            c: |b, c| c == 0 && b[c] == b[c + 4] && b[c] == b[c + 8],
            d: |b, c| c == 2 && b[c] == b[c + 2] && b[c] == b[c + 4],
        }
    }
    fn matches(&self, b: &[Option<C>], c: usize) -> Vec<WinLayout> {
        let mut v = vec![];

        if b[c].is_none() {
            return v;
        }

        use WinLayout::*;
        if (self.a)(b, c) {
            v.push(A);
        }
        if (self.b)(b, c) {
            v.push(B);
        }
        if (self.c)(b, c) {
            v.push(C);
        }
        if (self.d)(b, c) {
            v.push(D);
        }

        v
    }
}

enum WinLayout {
    A, // ---
    B, // |
    C, // \
    D, // /
}

impl WinLayout {
    fn apply_win(&self, b: &mut [Option<C>], c: usize) {
        let w = match b[c].unwrap() {
            crate::C::X => Some(crate::C::XWin),
            crate::C::O => Some(crate::C::OWin),
            // Also it can be XWIN or OWIN
            // No need to modify in that case
            c => Some(c),
        };
        use WinLayout::*;
        match self {
            A => {
                b[c] = w;
                b[c + 1] = w;
                b[c + 2] = w;
            }
            B => {
                b[c] = w;
                b[(c + 3) % 9] = w;
                b[(c + 6) % 9] = w;
            }
            C => {
                b[c] = w;
                b[c + 4] = w;
                b[c + 8] = w;
            }
            D => {
                b[c] = w;
                b[c + 2] = w;
                b[c + 4] = w;
            }
        }
    }
}

impl Board {
    fn check_for_end(&mut self) {
        // 0 1 2
        // 3 4
        // 6   8
        let mut winner = None;
        for c in 0..self.b.len() {
            let wls = self.rules.matches(&self.b, c);
            if !wls.is_empty() {
                wls.into_iter().for_each(|wl| wl.apply_win(&mut self.b, c));
                winner = self.b[c];
            }
        }
        if let Some(w) = winner {
            let msg = match w {
                C::X | C::XWin => "Human wins",
                C::O | C::OWin => "Bot wins",
            };

            self.render_board();

            println!("{}", msg);
            println!();
            std::process::exit(0);
        }

        if self.b.iter().all(|c| c.is_some()) {
            println!("Draw!");
            println!();
            std::process::exit(0);
        }
    }

    fn render_board(&self) {
        let s = std::io::stdout();
        let _s = s.lock();
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
        let s = std::io::stdin();
        let mut s = s.lock();
        while self.try_play(&mut s).is_none() {
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

    fn try_play(&mut self, s: &mut std::io::StdinLock) -> Option<()> {
        self.input.clear();
        s.read_line(&mut self.input).ok()?;

        let mut input = self.input.chars();
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
