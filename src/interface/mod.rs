use crate::{
    aux::{
        coord::Coord,
        index_iter::{IndexIterSigned, IndexIterUnsigned},
        ModuloSignedExt,
        DivFloorSignedExt,
    },
    game::{self, Game, AbsoluteCoord},
};

use std::{
    ops::{Add, Rem},
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
            unsafe { ncurses::COLS  } as usize,
            unsafe { ncurses::LINES } as usize,
        );
    }
    
    fn checker_color(&self, square: Coord<usize>) -> i16 {
        let modulo = self
            .scroll
            .add(square.into())
            .rem(Coord::squared(2))
            .sum()
            .modulo(2) as i16;
        
        CHECKER_1 + modulo
    }

    fn visible_chunks<'a>(&self, game: &'a Game) -> impl Iterator<Item=(Coord<isize>, &'a game::chunk::Chunk)> {
        let far_corner = self.scroll + Coord::from(self.size/Coord(2,1));

        let min = self.scroll.map(|x| x.div_floor(8)    );
        let max = far_corner .map(|x| x.div_floor(8) + 1);
        let dimension = (max - min).abs();

        IndexIterSigned::new(dimension, min).filter_map(move |coord|
            game.get_chunk(coord).map(|chunk| (coord, chunk))
        )
    }
    
    fn print_checkerboard(&self) {
        let checker_size = self.size / Coord(2,1);
        IndexIterUnsigned::new(checker_size, Coord::default())
            .map(|square| (square, self.checker_color(square)))
            .map(|(square, color)| (square * Coord(2,1), color))
            .for_each(|(Coord(x, y), color)|
                with_color(
                    color,
                    || { ncurses::mvaddstr(y as i32, x as i32, "  "); }
                )
            );
    }
    
    fn print_chunks(&self, game: &Game) {
        for (chunk, chunk_ref) in self.visible_chunks(game) {
            for square in game::chunk::all_squares() {
                let world_space = Coord::from(AbsoluteCoord { chunk, square });
                let screen_space = self.world_to_screen_space(world_space);

                let color = self.checker_color(screen_space / Coord(2, 1));
                
                let Coord(x, y) = screen_space.map(|x| x as i32);
                
                use self::game::SquareView::*;
                match chunk_ref.view(square) {
                    Unclicked  => with_color(color,     || { ncurses::mvaddstr(y, x, "  "); }),
                    Flagged    => with_color(color,     || { ncurses::mvaddstr(y, x, "/>"); }),
                    Penalty    => with_color(PENALTY,   || { ncurses::mvaddstr(y, x, "><"); }),
                    Points     => with_color(POINTS,    || { ncurses::mvaddstr(y, x, "<>"); }),
                    Clicked(n) => with_color(OVERLAY_1, || {
                        ncurses::mvaddch(y, x, ' ' as u64);
                        ncurses::mvaddch(
                            y, x+1,
                            if n == 0 { b' ' } else { (n + b'0') } as u64
                        );
                    }),
                }
            }
        };
    }

    fn print_overlay(&self, game: &Game) {
        let message = format!(
            "Solved: {} | Exploded: {} | Allocated: {}",
            game.chunks_won(),
            game.chunks_lost(),
            game.chunks.len(),
        );

        let Coord(x, y) = self.size - Coord(0, 1);
        with_color(OVERLAY_1,||{
            ncurses::mvaddstr(
                y as i32, 0,
                std::iter::repeat(' ').take(x).collect::<String>().as_str(),
            );
            ncurses::mvaddstr(
                y as i32, 2,
                message.as_str(),
            );
        });
    }
    
    fn mouse_click_event(&mut self, game: &mut Game) {
        let mut mouse_event: ncurses::MEVENT = unsafe { mem::uninitialized() };
        ncurses::getmouse(&mut mouse_event as *mut ncurses::MEVENT);
        
        let mouse_coord = Coord(mouse_event.x as usize, mouse_event.y as usize);
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
            KEY_UP    => Coord( 0, -1),
            KEY_DOWN  => Coord( 0,  1),
            KEY_LEFT  => Coord(-1,  0),
            KEY_RIGHT => Coord( 1,  0),
            _ => unreachable!(),
        };
    }
    
    fn screen_to_world_space(&self, coord: Coord<usize>) -> Coord<isize> {
        self.scroll + Coord::from(coord/Coord(2,1))
    }
    
    fn world_to_screen_space(&self, coord: Coord<isize>) -> Coord<usize> {
        ((coord - self.scroll) * Coord(2,1)).into()
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
