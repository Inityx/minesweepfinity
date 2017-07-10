use game::Game;
use ncurses::*;

#[derive(Default)]
pub struct Interface {
    scroll: (isize, isize),
    rows: usize,
    cols: usize,
    left_margin_width: usize,
    checker_cols: usize,
}

impl Interface {
    pub fn new() -> Interface {
        let mut ret = Default::default();
        
        initscr();  // create ncurses screen
        cbreak();   // enforce terminal cbreak mode
        start_color(); // initialize colors

        // overlay colors
        init_pair(20, COLOR_WHITE, COLOR_BLACK);
        init_pair(21, COLOR_GREEN, COLOR_BLACK);

        // checkerboard colors
        init_pair(10, COLOR_WHITE, COLOR_YELLOW);
        init_pair(11, COLOR_WHITE, COLOR_BLUE);
        
        // clicked colors
        for i in 0..9 {
            init_pair(i, COLOR_WHITE, COLOR_BLACK);
        }
        
        Self::resize(&mut ret);
        ret
    }

    pub fn play(&mut self, game: &mut Game) {
        self.render(&game)
    }
    
    fn resize(&mut self) {
        unsafe {
            self.rows = LINES as usize;
            self.cols = COLS as usize;
        }
        
        self.left_margin_width = ((self.rows-2) as f64).log(10.0) as usize + 1;
        self.checker_cols = (self.cols as usize-self.left_margin_width)/2;
    }
    
    fn render(&self, game: &Game) {
        self.print_checkerboard();
        self.print_chunks(game);
        self.print_overlay(game);
        
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        refresh();
    }
    
    pub fn print_checkerboard(&self) {
        for i in 1..self.rows {
            for j in 0..self.checker_cols {
                let row = i as isize + self.scroll.0;
                let col = j as isize + self.scroll.1;
                
                attron(
                    COLOR_PAIR(
                        ((row+col)%2+10) as i16
                    )
                );
                // mvaddstr(i,j*2,"５"); // ncurses no likey :(
                mvaddstr(
                    i as i32,
                    (j*2 + self.left_margin_width) as i32,
                    "  ",
                );
                attroff(
                    COLOR_PAIR(
                        ((row+col)%2+10) as i16
                    )
                );
            }
        }
    }
    
    #[allow(unused_variables)]
    pub fn print_chunks(&self, game: &Game) {
    }

    pub fn print_overlay(&self, game: &Game) {
        attron(COLOR_PAIR(20));
        
        // top LH corner
        mvaddstr(
            0, 0,
            format!(
                "{:>pad$}",
                ' ',
                pad=self.left_margin_width
            ).as_str(),
        );
        
        // column labels
        for i in 0..self.checker_cols {
            attron(COLOR_PAIR(21));
            mvaddch(
                0,
                (i*2+self.left_margin_width) as i32,
                ((i/26) as u8 + b'a') as u64,
            );
            
            attroff(COLOR_PAIR(21));
            mvaddch(
                0,
                (i*2+self.left_margin_width+1) as i32,
                ((i%26) as u8 + b'a') as u64,
            );
        }
        
        // row labels
        for i in 0..self.rows-2 {
            let string = format!(
                "{:>pad$}",
                i,
                pad=self.left_margin_width,
            );
            
            mvaddstr(
                (i+1) as i32,
                0,
                string.as_str(),
            );
        }

        // bottom bar
        for i in 0..self.checker_cols {
            mvaddstr(
                self.rows as i32-1,
                (i*2 + self.left_margin_width) as i32,
                "  ",
            );
        }
        
        mvaddstr(
            self.rows as i32-1,
            (self.left_margin_width + 2) as i32,
            format!(
                "Chunks solved: {}",
                game.get_chunks_won(),
            ).as_str(),
        );

        attroff(COLOR_PAIR(20));
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        endwin();   // destroy ncurses screen
    }
}
