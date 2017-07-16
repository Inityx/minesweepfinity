use std::mem;

use ncurses;
use ncurses::COLOR_PAIR;

use game::Game;
use game::SquareView::*;
use aux::coord::Coord;
use std::io::{stderr,Write};

const KEY_UPPER_A: i32 = b'A' as i32;
const KEY_LOWER_Z: i32 = b'z' as i32;
const KEY_ZERO:    i32 = b'0' as i32;
const KEY_NINE:    i32 = b'9' as i32;

#[derive(Default)]
pub struct Interface {
    // window: ncurses::WINDOW,
    scroll: Coord<isize>,
    size: Coord<usize>,
    margin: Coord<usize>,
    checker_cols: usize,
}

impl Interface {
    pub fn new() -> Interface {
        use ncurses::*;
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
        init_pair(10, COLOR_BLACK, COLOR_WHITE);
        init_pair(11, COLOR_WHITE, COLOR_GREEN);
        
        // clicked colors
        for i in 0..9 { init_pair(i, COLOR_WHITE, COLOR_BLACK); }
        
        let mut ret = Interface::default();
        Self::resize(&mut ret);
        ret
    }

    pub fn play(&mut self, mut game: &mut Game) {
        self.render(&game);
        loop {
            let character = ncurses::getch();
            match character {
                ncurses::KEY_RESIZE => self.resize(),
                ncurses::KEY_MOUSE => self.mouse_click_event(&mut game),
                KEY_UPPER_A...KEY_LOWER_Z |
                    KEY_ZERO...KEY_NINE => self.alpha_key_event(character),
                ncurses::KEY_DOWN...ncurses::KEY_RIGHT => self.arrow_key_event(character),
                _ => ()
            }
            self.render(&game);
        }
    }
    
    fn resize(&mut self) {
        unsafe {
            self.size = Coord(
                ncurses::LINES as usize,
                ncurses::COLS as usize,
            );
        }
        
        self.margin = Coord(1, ((self.size.0-2) as f64).log(10.0) as usize + 1);
        self.checker_cols = (self.size.1 as usize-self.margin.1)/2;
    }
    
    fn render(&self, game: &Game) {
        self.print_checkerboard();
        self.print_chunks(game);
        self.print_overlay(game);
        
        ncurses::refresh();
    }
    
    fn print_checkerboard(&self) { // TODO debug extra printing
        for i in 1..self.size.0 {
            for j in 0..self.checker_cols {
                let row = i as isize + self.scroll.0%2;
                let col = j as isize + self.scroll.1%2;
                let color = COLOR_PAIR( ((row+col)%2+10) as i16 );
                
                ncurses::attron(color);
                // mvaddstr(i,j*2,"５"); // ncurses no likey :(
                ncurses::mvaddstr(
                    i as i32,
                    (j*2 + self.margin.1) as i32,
                    "  ",
                );
                ncurses::attroff(color);
            }
        }
    }
    
    fn print_chunks(&self, game: &Game) {
        let visible_chunks = ::aux::index_iter::self_and_adjacent() // TODO actually compute this value
            .filter_map( |chunk_location|
                game.get_chunk(chunk_location)
                    .and_then(|chunk| Some((chunk_location, chunk)) )
            );
            
        ncurses::attron(COLOR_PAIR(20));
        for (chunk_location, chunk) in visible_chunks {
            for (index, view) in chunk.view().into_iter().enumerate() {
                
                let real_space = chunk_location*8 + Coord::from(Coord(index/8, index%8));
                let screen_space = self.to_screen_space(real_space);
                
                let row = screen_space.0 as i32;
                let col = screen_space.1 as i32;
                
                match view {
                    Clicked(neighbors) => {
                        if neighbors == 0 {
                            ncurses::mvaddstr(row, col, "  ");
                        } else {
                            ncurses::mvaddch(row, col,   ' ' as u64);
                            ncurses::mvaddch(row, col+1, (neighbors + b'0') as u64);
                        }
                    },
                    Unclicked {flag, mine} => {
                        if flag || mine {
                            ncurses::mvaddch(row, col,   (if mine {'#'} else {' '}) as u64);
                            ncurses::mvaddch(row, col+1, (if flag {'F'} else {' '}) as u64);
                        }
                    }
                }
            }
        }
        ncurses::attroff(COLOR_PAIR(20));
    }

    fn print_overlay(&self, game: &Game) {
        ncurses::attron(COLOR_PAIR(20));
        
        // top LH corner
        ncurses::mvaddstr(
            0, 0,
            format!(
                "{:>pad$}",
                ' ',
                pad=self.margin.1
            ).as_str(),
        );
        
        // column labels
        let char_from_index = |character| (character as u8 + b'a');
        for i in 0..self.checker_cols {
            let col = (i*2+self.margin.1) as i32;
            
            ncurses::attron(COLOR_PAIR(21));
            ncurses::mvaddch(0, col, char_from_index(i/26) as u64);
            
            ncurses::attroff(COLOR_PAIR(21));
            ncurses::mvaddch(0, col+1, char_from_index(i%26) as u64);
        }
        
        // row labels
        for i in 0..((self.size.0 as i32) - 2) {
            let string = format!(
                "{:>pad$}", i,
                pad = self.margin.1,
            );
            
            ncurses::mvaddstr(i+1, 0, string.as_str());
        }

        // bottom bar
        for i in 0..(self.checker_cols as i32) {
            ncurses::mvaddstr(
                (self.size.0 as i32) - 1,
                i*2 + self.margin.1 as i32,
                "  ",
            );
        }
        
        let won_message = format!(
            "Chunks solved: {} | Scroll: {} | Chunks allocated: {}",
            game.get_chunks_won(),
            self.scroll,
            game.get_allocations()
        );
        ncurses::mvaddstr(
            (self.size.0 as i32) - 1,
            (self.margin.1 as i32) + 2,
            won_message.as_str(),
        );

        ncurses::attroff(COLOR_PAIR(20));
    }
    
    fn mouse_click_event(&mut self, game: &mut Game) {
        use ncurses::MEVENT;
        let mut mouse_event: MEVENT = unsafe { mem::uninitialized() };
        ncurses::getmouse(&mut mouse_event as *mut MEVENT);
        let mouse_coord = Coord(mouse_event.y as usize, mouse_event.x as usize);
        let real_coord = self.to_world_space(Coord::from(mouse_coord));
        
        writeln!(stderr(),"Mouse event at {}", real_coord).unwrap();
        game.touch(real_coord);
    }
    
    fn alpha_key_event(&mut self, character: i32) {
        ncurses::mvaddstr(
            0, 0,
            format!("Key event {}", character as u8 as char).as_str()
        );
    }
    
    fn arrow_key_event(&mut self, arrow: i32) {
        self.scroll += match arrow {
            ncurses::KEY_UP    => Coord( 1,  0),
            ncurses::KEY_DOWN  => Coord(-1,  0),
            ncurses::KEY_LEFT  => Coord( 0,  1),
            ncurses::KEY_RIGHT => Coord( 0, -1),
            _ => unreachable!(),
        };
    }
    
    fn to_world_space(&self, coord: Coord<isize>) -> Coord<isize> {
        (coord - Coord::from(self.margin))/Coord(1,2) - self.scroll
    }
    
    fn to_screen_space(&self, coord: Coord<isize>) -> Coord<isize> {
        (coord + self.scroll)*Coord(1,2) + Coord::from(self.margin)
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
