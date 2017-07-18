use std::mem;

use ncurses;
use ncurses::COLOR_PAIR;

use game::Game;
use game::SquareView::*;
use aux::coord::Coord;
use std::io::{Write,stderr};

const KEY_UPPER_A: i32 = b'A' as i32;
const KEY_LOWER_Z: i32 = b'z' as i32;
const KEY_ZERO:    i32 = b'0' as i32;
const KEY_NINE:    i32 = b'9' as i32;

const CHECKER_1: i16 = 10;
const CHECKER_2: i16 = 11;
const OVERLAY_1: i16 = 20;
const OVERLAY_2: i16 = 21;

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
        mousemask(ALL_MOUSE_EVENTS as mmask_t, None);
        mouseinterval(0);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        start_color(); // initialize colors

        // overlay colors
        init_pair(OVERLAY_1, COLOR_WHITE, COLOR_BLACK);
        init_pair(OVERLAY_2, COLOR_GREEN, COLOR_BLACK);

        // checkerboard colors
        init_pair(CHECKER_1, COLOR_BLACK, COLOR_WHITE);
        init_pair(CHECKER_2, COLOR_BLACK, COLOR_GREEN);
        
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
    
    fn checker_color(&self, row: isize, col: isize) -> i16 {
        (
            (row   + self.scroll.0 % 2) +
            (col/2 + self.scroll.1 % 2)
        ) as i16 % 2 + CHECKER_1
    }
    
    fn print_checkerboard(&self) { // TODO debug extra printing
        for row in self.margin.0..self.size.0 {
            for col in self.margin.1..self.size.1 {
                let color = COLOR_PAIR(
                    self.checker_color(
                        row as isize,
                        col as isize
                    )
                );
                
                ncurses::attron(color);
                // mvaddstr(i,j*2,"５"); // ncurses no likey :(
                ncurses::mvaddch(
                    row as i32,
                    col as i32,
                    ' ' as u64,
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
            
        ncurses::attron(COLOR_PAIR(OVERLAY_1));
        for (chunk_location, chunk) in visible_chunks {
            for (index, view) in chunk.view().into_iter().enumerate() {
                
                let world_space = chunk_location*8 + Coord::from(Coord(index/8, index%8));
                let screen_space = self.world_to_screen_space(world_space);
                
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
                            let color = self.checker_color(row as isize, col as isize);
                            ncurses::attron(COLOR_PAIR(color));
                            ncurses::mvaddch(row, col,   (if mine {'#'} else {' '}) as u64);
                            ncurses::mvaddch(row, col+1, (if flag {'F'} else {' '}) as u64);
                            ncurses::attroff(COLOR_PAIR(color));
                        }
                    }
                }
            }
        }
        ncurses::attroff(COLOR_PAIR(OVERLAY_1));
    }

    fn print_overlay(&self, game: &Game) {
        ncurses::attron(COLOR_PAIR(OVERLAY_1));
        
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
            
            ncurses::attron(COLOR_PAIR(OVERLAY_2));
            ncurses::mvaddch(0, col, char_from_index(i/26) as u64);
            
            ncurses::attroff(COLOR_PAIR(OVERLAY_2));
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
        let row = (self.size.0 - 1) as i32;
        for col in 0..(self.size.1 as i32) {
            ncurses::mvaddch(row, col, ' ' as u64);
        }
        
        let won_message = format!(
            "Solved: {} | Scroll: {} | Chunks: {}",
            game.get_chunks_won(),
            self.scroll,
            game.get_allocations()
        );
        ncurses::mvaddstr(
            (self.size.0 as i32) - 1,
            (self.margin.1 as i32) + 2,
            won_message.as_str(),
        );

        ncurses::attroff(COLOR_PAIR(OVERLAY_1));
    }
    
    fn mouse_click_event(&mut self, game: &mut Game) {
        use ncurses::MEVENT;
        let mut mouse_event: MEVENT = unsafe { mem::uninitialized() };
        ncurses::getmouse(&mut mouse_event as *mut MEVENT);
        
        let mouse_coord = Coord(mouse_event.y as usize, mouse_event.x as usize);
        let real_coord = self.screen_to_world_space(Coord::from(mouse_coord));
        
        if (mouse_event.bstate & ncurses::BUTTON1_PRESSED as ncurses::mmask_t) != 0 {
            game.touch(real_coord);
        } else if (mouse_event.bstate & ncurses::BUTTON3_PRESSED as ncurses::mmask_t) != 0 {
            game.toggle_flag(real_coord);
        }
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
    
    fn screen_to_world_space(&self, coord: Coord<isize>) -> Coord<isize> {
        (coord - Coord::from(self.margin))/Coord(1,2) - self.scroll
    }
    
    fn world_to_screen_space(&self, coord: Coord<isize>) -> Coord<isize> {
        (coord + self.scroll)*Coord(1,2) + Coord::from(self.margin)
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}
