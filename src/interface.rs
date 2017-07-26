use std::mem;

use ncurses;
use ncurses::COLOR_PAIR;

use game::Game;
use game::SquareView::*;
use aux::coord::Coord;
use aux::index_iter::IndexIterSigned;

use std::thread;
use std::time::Duration;
use std::io::Write;

const CHECKER_1: i16 = 10;
const CHECKER_2: i16 = 11;
const OVERLAY_1: i16 = 20;
const OVERLAY_2: i16 = 21;
const POINTS:    i16 =  8;
const PENALTY:   i16 =  9;

const SPREAD_DELAY_MS: u64 = 30;

#[derive(Default)]
pub struct Interface {
    scroll: Coord<isize>,
    size: Coord<usize>,
    checker_cols: usize,
    spread_delay: Duration,
}

impl Interface {
    pub fn new() -> Interface {
        use ncurses::*;
        
        let window = initscr();
        
        cbreak();
        keypad(window, true);
        mousemask(ALL_MOUSE_EVENTS as mmask_t, None);
        mouseinterval(0);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        
        start_color();
        for i in 0..POINTS { init_pair(i, COLOR_WHITE, COLOR_BLACK); }
        init_pair(POINTS,  COLOR_BLACK, COLOR_YELLOW);
        init_pair(PENALTY, COLOR_BLUE,  COLOR_RED);
        init_pair(OVERLAY_1, COLOR_WHITE, COLOR_BLACK);
        init_pair(OVERLAY_2, COLOR_GREEN, COLOR_BLACK);
        init_pair(CHECKER_1, COLOR_BLACK, COLOR_WHITE);
        init_pair(CHECKER_2, COLOR_BLACK, COLOR_GREEN);
        
        let mut ret = Interface::default();
        ret.spread_delay = Duration::from_millis(SPREAD_DELAY_MS);
        ret.resize();
        return ret;
    }

    pub fn play(&mut self, mut game: Game) {
        self.render(&game);
        loop {
            let character = ncurses::getch();
            match character {
                ncurses::KEY_RESIZE => self.resize(),
                ncurses::KEY_MOUSE => self.mouse_click_event(&mut game),
                ncurses::KEY_DOWN...ncurses::KEY_RIGHT => self.arrow_key_event(character),
                _ => ()
            }
            self.render(&game);
        }
    }
    
    fn resize(&mut self) {
        self.size = unsafe {
            Coord(
                ncurses::LINES as usize,
                ncurses::COLS as usize,
            )
        };
        
        self.checker_cols = self.size.1/2;
    }
    
    fn render(&self, game: &Game) {
        self.print_checkerboard();
        self.print_chunks (game);
        self.print_overlay(game);
        
        ncurses::refresh();
    }

    fn checker_color(&self, row: usize, col: usize) -> i16 {
        (
            (row     as isize) + self.scroll.0 % 2 +
            (col as isize / 2) + self.scroll.1 % 2
        ) as i16 % 2 + CHECKER_1
    }

    fn visible_chunk_coords(&self) -> IndexIterSigned {
        let far_corner = self.scroll + Coord::from(self.size/Coord(1,2));

        let min_modulus_offset = Coord((self.scroll.0 < 0) as isize, (self.scroll.1 < 0) as isize);
        let max_modulus_offset = Coord((far_corner.0 >= 0) as isize, (far_corner.1 >= 0) as isize);
        
        let min: Coord<isize> = self.scroll/8 - min_modulus_offset;
        let max: Coord<isize> = far_corner/8  + max_modulus_offset;
        let dimension = (max - min).abs();

        IndexIterSigned::new(dimension, min)
    }
    
    fn print_checkerboard(&self) { // TODO debug extra printing
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                with_color(
                    self.checker_color(row, col),
                    ||{ ncurses::mvaddch(row as i32, col as i32, ' ' as u64); }
                );
            }
        }
    }
    
    fn print_chunks(&self, game: &Game) {
        let visible_chunks = self.visible_chunk_coords()
            .filter_map( |chunk_location|
                game.get_chunk(chunk_location).and_then(|chunk|
                    Some((chunk_location, chunk))
                )
            );
            
        ncurses::attron(COLOR_PAIR(OVERLAY_1));
        for (chunk_location, chunk) in visible_chunks {
            for (index, view) in chunk.view().into_iter().enumerate() {
                
                let world_space = chunk_location*8 + Coord::from(Coord(index/8, index%8));
                let screen_space = self.world_to_screen_space(world_space);
                
                let row = screen_space.0 as i32;
                let col = screen_space.1 as i32;
                
                let color = self.checker_color(row as usize, col as usize);
                
                match view {
                    Unclicked => with_color(color,   ||{ ncurses::mvaddstr(row, col, "  "); }),
                    Flagged   => with_color(color,   ||{ ncurses::mvaddstr(row, col, "/>"); }),
                    Penalty   => with_color(PENALTY, ||{ ncurses::mvaddstr(row, col, "><"); }),
                    Points    => with_color(POINTS,  ||{ ncurses::mvaddstr(row, col, "<>"); }),
                    Clicked(neighbors) => {
                        ncurses::mvaddch(row, col, ' ' as u64);
                        ncurses::mvaddch(
                            row, col+1,
                            if neighbors == 0 { b' ' } else { (neighbors + b'0') } as u64
                        );
                    },
                }
            }
        }
        ncurses::attroff(COLOR_PAIR(OVERLAY_1));
    }

    fn print_overlay(&self, game: &Game) {
        ncurses::attron(COLOR_PAIR(OVERLAY_1));
        
        let row = (self.size.0 - 1) as i32;
        for col in 0..(self.size.1 as i32) {
            ncurses::mvaddch(row, col, ' ' as u64);
        }
        
        let message = format!(
            "Solved: {} | Exploded: {} | Scroll: {}",
            game.get_chunks_won(),
            game.get_chunks_lost(),
            self.scroll
        );
        ncurses::mvaddstr(
            (self.size.0 as i32) - 1, 2,
            message.as_str(),
        );

        ncurses::attroff(COLOR_PAIR(OVERLAY_1));
    }
    
    fn mouse_click_event(&mut self, game: &mut Game) {
        let mut mouse_event: ncurses::MEVENT = unsafe { mem::uninitialized() };
        ncurses::getmouse(&mut mouse_event as *mut ncurses::MEVENT);
        
        let mouse_coord = Coord(mouse_event.y as usize, mouse_event.x as usize);
        let real_coord = self.screen_to_world_space(Coord::from(mouse_coord));
        
        if (mouse_event.bstate & ncurses::BUTTON1_PRESSED as ncurses::mmask_t) != 0 {
            // Spreading click cascade
            let mut to_click = vec![real_coord];
            
            while let Some(fringe) = game.touch(to_click) {
                self.print_chunks (game);
                self.print_overlay(game);
                ncurses::refresh();
                to_click = fringe;
                thread::sleep(self.spread_delay);
            }
        } else if (mouse_event.bstate & ncurses::BUTTON3_PRESSED as ncurses::mmask_t) != 0 {
            game.toggle_flag(real_coord);
        }
    }
    
    fn arrow_key_event(&mut self, arrow: i32) {
        self.scroll += match arrow {
            ncurses::KEY_UP    => Coord(-1,  0),
            ncurses::KEY_DOWN  => Coord( 1,  0),
            ncurses::KEY_LEFT  => Coord( 0, -1),
            ncurses::KEY_RIGHT => Coord( 0,  1),
            _ => unreachable!(),
        };
    }
    
    fn screen_to_world_space(&self, coord: Coord<isize>) -> Coord<isize> {
        coord/Coord(1,2) + self.scroll
    }
    
    fn world_to_screen_space(&self, coord: Coord<isize>) -> Coord<isize> {
        (coord - self.scroll)*Coord(1,2)
    }
}

impl Drop for Interface {
    fn drop(&mut self) {
        ncurses::endwin();
    }
}

#[inline]
fn with_color<F>(color: i16, func: F) where F: Fn() {
    ncurses::attron(COLOR_PAIR(color));
    func();
    ncurses::attroff(COLOR_PAIR(color));
}
