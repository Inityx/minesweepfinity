use std::vec::Vec;
use std::mem;

use ncurses::*;

use game::Game;
use game::chunk::SquareView;
use aux::coord::Coord;

pub struct Interface {
    window: WINDOW,
    scroll: Coord<isize>,
    size: Coord<usize>,
    left_margin_width: usize,
    checker_cols: usize,
}

impl Interface {
    pub fn new() -> Interface {
        let window = initscr();  // create ncurses screen
        cbreak();   // enforce terminal cbreak mode
        keypad(window, true);
        mousemask(NCURSES_BUTTON_CLICKED as mmask_t, None);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        start_color(); // initialize colors

        // overlay colors
        init_pair(20, COLOR_WHITE, COLOR_BLACK);
        init_pair(21, COLOR_GREEN, COLOR_BLACK);

        // checkerboard colors
        init_pair(10, COLOR_WHITE, COLOR_YELLOW);
        init_pair(11, COLOR_WHITE, COLOR_BLUE);
        
        // clicked colors
        for i in 0..9 { init_pair(i, COLOR_WHITE, COLOR_BLACK); }
        
        let mut ret = Interface {
            window: window,
            scroll: Default::default(),
            size: Default::default(),
            left_margin_width: Default::default(),
            checker_cols: Default::default(),
        };
        Self::resize(&mut ret);
        ret
    }

    pub fn play(&mut self, game: &mut Game) {
        self.render(&game);
        loop {
            match getch() {
                KEY_RESIZE => { self.resize(); self.render(&game); },
                KEY_MOUSE => self.mouse_click_event(),
                KEY_CODE_YES => self.alpha_key_event(),
                _ => (),
            }
        }
    }
    
    fn resize(&mut self) {
        unsafe {
            self.size.0 = LINES as usize;
            self.size.1 = COLS as usize;
        }
        
        self.left_margin_width = ((self.size.0-2) as f64).log(10.0) as usize + 1;
        self.checker_cols = (self.size.1 as usize-self.left_margin_width)/2;
    }
    
    fn render(&self, game: &Game) {
        self.print_checkerboard();
        self.print_chunks(game);
        self.print_overlay(game);
        
        refresh();
    }
    
    pub fn print_checkerboard(&self) { // TODO debug extra printing
        for i in 1..self.size.0 {
            for j in 0..self.checker_cols {
                let row = i as isize + self.scroll.0;
                let col = j as isize + self.scroll.1;
                let color = COLOR_PAIR( ((row+col)%2+10) as i16 );
                
                attron(color);
                // mvaddstr(i,j*2,"５"); // ncurses no likey :(
                mvaddstr(
                    i as i32,
                    (j*2 + self.left_margin_width) as i32,
                    "  ",
                );
                attroff(color);
            }
        }
    }
    
    #[allow(unused_variables)]
    pub fn print_chunks(&self, game: &Game) {
        let n: Vec<SquareView> = game.view_chunk(&Coord(0,0));
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
        let char_from_index = |character| (character as u8 + b'a');
        for i in 0..self.checker_cols {
            let col = (i*2+self.left_margin_width) as i32;
            
            attron(COLOR_PAIR(21));
            mvaddch(0, col, char_from_index(i/26) as u64);
            
            attroff(COLOR_PAIR(21));
            mvaddch(0, col+1, char_from_index(i%26) as u64);
        }
        
        // row labels
        for i in 0..((self.size.0 as i32) - 2) {
            let string = format!(
                "{:>pad$}", i,
                pad = self.left_margin_width,
            );
            
            mvaddstr(i+1, 0, string.as_str());
        }

        // bottom bar
        for i in 0..(self.checker_cols as i32) {
            mvaddstr(
                (self.size.0 as i32) - 1,
                i*2 + self.left_margin_width as i32,
                "  ",
            );
        }
        
        let won_message = format!(
            "Chunks solved: {}, Size: {:?}, Cols: {}",
            game.get_chunks_won(),
            self.size,
            self.checker_cols,
        );
        mvaddstr(
            (self.size.0 as i32) - 1,
            (self.left_margin_width as i32) + 2,
            won_message.as_str(),
        );

        attroff(COLOR_PAIR(20));
    }
    
    fn mouse_click_event(&mut self) {
        let mut mouse_event: MEVENT = unsafe { mem::uninitialized() };
        getmouse(&mut mouse_event as *mut MEVENT);
        let mouse_coord = Coord(mouse_event.y as usize, mouse_event.x as usize);
        mvaddstr(
            0, 0,
            format!("Mouse event at {:?}", mouse_coord).as_str()
        );
    }
    
    fn alpha_key_event(&mut self) {
        
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        endwin();
    }
}
