use crate::{
    aux::{
        coord::Coord,
        index_iter::{IndexIterSigned, IndexIterUnsigned},
        ModuloSignedExt,
        DivFloorSignedExt,
    },
    game::{
        Game,
        SquareView::*,
    },
};

use std::{
    thread,
    time::Duration,
    mem,
};

use ncurses::{self, COLOR_PAIR};

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
        init_pair(PENALTY, COLOR_YELLOW,  COLOR_RED);
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
        self.render_full(&game);
        
        loop {
            match ncurses::getch() {
                ncurses::KEY_MOUSE => {
                    self.mouse_click_event(&mut game);
                    self.render_partial(&game);
                },
                ncurses::KEY_RESIZE => {
                    self.resize();
                    self.render_full(&game);
                },
                character @ ncurses::KEY_DOWN..=ncurses::KEY_RIGHT => {
                    self.arrow_key_event(character);
                    self.render_full(&game);
                },
                _ => ()
            }
        }
    }

    fn render_partial(&self, game: &Game) {
        self.print_chunks(game);
        self.print_overlay(game);
        ncurses::refresh();
    }

    fn render_full(&self, game: &Game) {
        self.print_checkerboard();
        self.print_chunks(game);
        self.print_overlay(game);
        ncurses::refresh();
    }
    
    fn resize(&mut self) {
        self.size = Coord(
            // Safe because extern statics are not being modified
            unsafe { ncurses::LINES } as usize,
            unsafe { ncurses::COLS } as usize,
        );
        
        self.checker_cols = self.size.1/2;
    }
    
    fn checker_color(&self, coord: Coord<usize>) -> i16 {
        let checker_coord = coord/Coord(1,2);
        (
            self.scroll.map(|x| x % 2).sum() +
            checker_coord.map(|x| x % 2).sum() as isize
        ).modulo(2) as i16 + CHECKER_1
    }

    fn visible_chunk_coords(&self) -> IndexIterSigned {
        let far_corner = self.scroll + Coord::from(self.size/Coord(1,2));

        let min: Coord<isize> = self.scroll.map(|x| x.div_floor(8));
        let max: Coord<isize> = far_corner.map(|x| x.div_floor(8) + 1);
        let dimension = (max - min).abs();

        IndexIterSigned::new(dimension, min)
    }
    
    fn print_checkerboard(&self) {
        let checker_size = self.size / Coord(1,2);
        IndexIterUnsigned::new(checker_size, Coord(0,0))
            .map(|coord| coord * Coord(1,2))
            .for_each(|square|
                with_color(
                    self.checker_color(square),
                    || { ncurses::mvaddstr(square.0 as i32, square.1 as i32, "  "); }
                )
            );
    }
    
    fn print_chunks(&self, game: &Game) {
        let visible_chunks = self
            .visible_chunk_coords()
            .filter_map(|chunk_location|
                game
                    .chunks()
                    .get(&chunk_location)
                    .map(|chunk| (chunk_location, chunk))
            );
            
        for (location, chunk) in visible_chunks {
            for (index, view) in chunk.view().into_iter().enumerate() {
                let world_space = location.map(|x| x * 8) + Coord::from(Coord(index/8, index%8));
                let screen_space = self.world_to_screen_space(world_space);

                let color = self.checker_color(screen_space);
                
                let row = screen_space.0 as i32;
                let col = screen_space.1 as i32;
                
                match view {
                    Unclicked => with_color(color,   ||{ ncurses::mvaddstr(row, col, "  "); }),
                    Flagged   => with_color(color,   ||{ ncurses::mvaddstr(row, col, "/>"); }),
                    Penalty   => with_color(PENALTY, ||{ ncurses::mvaddstr(row, col, "><"); }),
                    Points    => with_color(POINTS,  ||{ ncurses::mvaddstr(row, col, "<>"); }),
                    Clicked(neighbors) => with_color(
                        OVERLAY_1,
                        || {
                            ncurses::mvaddch(row, col, ' ' as u64);
                            ncurses::mvaddch(
                                row, col+1,
                                if neighbors == 0 { b' ' } else { (neighbors + b'0') } as u64
                            );
                        }
                    ),
                }
            }
        }
    }

    fn print_overlay(&self, game: &Game) {
        let row = (self.size.0 - 1) as i32;
        let message = format!(
            "Solved: {} | Exploded: {} | Scroll: {}",
            game.chunks_won(),
            game.chunks_lost(),
            self.scroll
        );

        with_color(OVERLAY_1,||{
            for col in 0..(self.size.1 as i32) {
                ncurses::mvaddch(row, col, ' ' as u64);
            }
            ncurses::mvaddstr(
                (self.size.0 as i32) - 1, 2,
                message.as_str(),
            );
        });
    }
    
    fn mouse_click_event(&mut self, game: &mut Game) {
        let mut mouse_event: ncurses::MEVENT = unsafe { mem::uninitialized() };
        ncurses::getmouse(&mut mouse_event as *mut ncurses::MEVENT);
        
        let mouse_coord = Coord(mouse_event.y as usize, mouse_event.x as usize);
        let real_coord = self.screen_to_world_space(mouse_coord);
        
        if (mouse_event.bstate & ncurses::BUTTON1_PRESSED as ncurses::mmask_t) != 0 {
            // Spreading click cascade
            let mut to_click = vec![real_coord];
            
            while let Some(fringe) = game.touch(&to_click) {
                to_click = fringe;
                self.print_chunks (game);
                self.print_overlay(game);
                ncurses::refresh();
                thread::sleep(self.spread_delay);
            }
        } else if (mouse_event.bstate & ncurses::BUTTON3_PRESSED as ncurses::mmask_t) != 0 {
            game.toggle_flag(real_coord);
        }
    }
    
    fn arrow_key_event(&mut self, arrow: i32) {
        use ncurses::*;
        self.scroll += match arrow {
            KEY_UP    => Coord(-1,  0),
            KEY_DOWN  => Coord( 1,  0),
            KEY_LEFT  => Coord( 0, -1),
            KEY_RIGHT => Coord( 0,  1),
            _ => unreachable!(),
        };
    }
    
    fn screen_to_world_space(&self, coord: Coord<usize>) -> Coord<isize> {
        self.scroll + Coord::from(coord/Coord(1,2))
    }
    
    fn world_to_screen_space(&self, coord: Coord<isize>) -> Coord<usize> {
        Coord::from((coord - self.scroll) * Coord(1,2))
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
