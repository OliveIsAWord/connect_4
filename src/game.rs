pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;
pub const BOARD_SIZE: usize = WIDTH * HEIGHT;
pub const BITMAP_SIZE: usize = WIDTH * (HEIGHT + 1);
pub const BITMAP_SIZE_BYTES: usize = (BITMAP_SIZE + 8 - 1) / 8; // Division rounding up

// NOTE: Bitmap must have at least as many bits as BITMAP_SIZE.
pub type Bitmap = u64;
// NOTE: Score must support all values (-BOARD_SIZE / 2, BOARD_SIZE]
pub type Score = i8;
pub const MAX_SCORE: Score = BOARD_SIZE as Score + 1;
pub const MIN_SCORE: Score = -MAX_SCORE;

// NOTE: For a struct implementing Copy, an implicit Copy is identical to an explicit Clone.
// At opt-level >= 2, they will be indistinguishable machine code
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Position {
    moves: Score,
    board: Bitmap,
    mask: Bitmap,
}

impl Position {
    pub fn new() -> Self {
        let moves = 0;
        let board = 0;
        let mask = 0;
        Position { moves, board, mask }
    }

    pub fn from_moves(pos: &str) -> Self {
        let mut this = Self::new();
        for c in pos.bytes() {
            let n = c as isize - '1' as isize; // maps from ASCII 1-7 to int 0-6
            if n < 0 || n >= WIDTH as isize {
                panic!("invalid character '{}'", c as char);
            }
            let n = n as usize;
            if this.can_play(n) {
                this.play(n);
            } else {
                panic!("Too many stones in column {}. Board:\n{}", n + 1, this);
            }
        }
        this
    }

    #[allow(dead_code)]
    pub fn from_key(pos: Bitmap) -> Self {
        let pos = pos + Self::all_bottom_mask();
        let mut this = Self::new();
        let mut empty = true;
        for i in 0..BITMAP_SIZE {
            let bit = (HEIGHT - i % WIDTH) + (i / WIDTH) * WIDTH;
            let bit_mask = 1 << bit;
            if empty {
                empty = pos & bit_mask == 0;
            } else {
                this.mask |= bit_mask;
                this.board |= pos & bit_mask;
                this.moves += 1;
            }
            if (i + 1) % WIDTH == 0 {
                empty = true;
            }
        }
        this
    }

    pub fn num_moves(&self) -> Score {
        self.moves
    }

    pub fn get_color(&self) -> bool {
        // true if an X was just played
        // this is population count but just returning even/odd
        self.mask.count_ones() % 2 == 1
    }

    pub fn can_play(&self, x: usize) -> bool {
        self.mask & Self::top_mask(x) == 0
    }

    pub fn play(&mut self, x: usize) {
        self.board ^= self.mask;
        self.mask |= self.mask + Self::bottom_mask(x);
        self.moves += 1;
    }

    #[allow(dead_code)]
    pub fn unplay(&mut self) {
        panic!("not implemented");
    }

    #[allow(dead_code)]
    pub fn unplay_row(&mut self, _x: usize) {
        panic!("not implemented");
    }

    pub fn is_winning_move(&self, x: usize) -> bool {
        let mut pos = self.board;
        pos |= (self.mask + Self::bottom_mask(x)) & Self::column_mask(x);
        Self::alignment(pos)
    }

    fn alignment(pos: Bitmap) -> bool {
        // horizontal
        let m = pos & (pos >> (HEIGHT + 1));
        if m & (m >> (2 * HEIGHT + 2)) > 0 {
            return true;
        }

        // diagonal 1
        let m = pos & (pos >> HEIGHT);
        if m & (m >> (2 * HEIGHT)) > 0 {
            return true;
        }

        // diagonal 2
        let m = pos & (pos >> (HEIGHT + 2));
        if m & (m >> (2 * HEIGHT + 4)) > 0 {
            return true;
        }

        // vertical
        let m = pos & (pos >> 1);
        if m & (m >> 2) > 0 {
            return true;
        }
        false
    }

    pub fn board_to_string(&self) -> String {
        let mut raw = Vec::<u8>::new();
        for i in 0..BOARD_SIZE {
            let bit = (HEIGHT - 1 - i / WIDTH) + (i % WIDTH) * WIDTH;
            let b = (self.board & (1 << bit) > 0) ^ self.get_color();
            let m = self.mask & (1 << bit) > 0;
            raw.push(if m {
                if b {
                    b'X'
                } else {
                    b'O'
                }
            } else {
                b'.'
            });
            if (i + 1) % WIDTH == 0 && i < BOARD_SIZE - 1 {
                raw.push(b'\n');
            }
        }
        String::from_utf8(raw).unwrap()
    }

    pub fn get_key(&self) -> Bitmap {
        self.board + self.mask
    }

    fn top_mask(x: usize) -> Bitmap {
        (1 << (HEIGHT - 1)) << (x * (HEIGHT + 1))
    }

    fn bottom_mask(x: usize) -> Bitmap {
        1 << (x * (HEIGHT + 1))
    }

    fn column_mask(x: usize) -> Bitmap {
        ((1 << HEIGHT) - 1) << (x * (HEIGHT + 1))
    }

    fn all_bottom_mask() -> Bitmap {
        (0..WIDTH).map(Self::bottom_mask).sum()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Moves: {}\nColor: {}\n{}",
            self.moves,
            if self.get_color() { 'O' } else { 'X' },
            self.board_to_string()
        )
    }
}
