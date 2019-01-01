use crate::aux::coord::Coord;
use std::ops::{BitAnd, Not, Shr};

pub const DIMENSION: usize = 8;

const BIT: u8 = 0b1;

#[inline]
fn bit_shift(index: usize) -> u8 {
    debug_assert!(index < DIMENSION);
    BIT << index
}

#[derive(Default, PartialEq, Clone, Copy)]
struct BitRow(u8);

impl BitRow {
    fn set(&mut self, index: usize) {
        self.0 |= bit_shift(index);
    }
    fn unset(&mut self, index: usize) {
        self.0 &= !bit_shift(index);
    }
    fn toggle(&mut self, index: usize) {
        self.0 ^= bit_shift(index);
    }
    fn get(&self, index: usize) -> bool {
        (self.0 & bit_shift(index)) != 0
    }
}

#[derive(Default, PartialEq, Clone, Copy)]
pub struct BitField([BitRow; DIMENSION]);

impl BitField {
    pub fn set(&mut self, Coord(col, row): Coord<usize>) {
        self.0[row].set(col);
    }
    pub fn unset(&mut self, Coord(col, row): Coord<usize>) {
        self.0[row].unset(col);
    }
    pub fn toggle(&mut self, Coord(col, row): Coord<usize>) {
        self.0[row].toggle(col);
    }
    pub fn get(&self, Coord(col, row): Coord<usize>) -> bool {
        self.0[row].get(col)
    }
}

impl Not for BitField {
    type Output = Self;
    fn not(mut self) -> Self {
        for BitRow(x) in &mut self.0 { *x = !*x; }
        self
    }
}

const NYBBLE: u32 = 0b1111;

#[inline]
fn nybble_shift(value: u8, index: usize) -> u32 {
    debug_assert!(index < DIMENSION);
    let index = index * 4;

    let value = value as u32;
    debug_assert!(value <= NYBBLE);

    value << index
}

#[inline]
fn nybble_unshift(value: u32, index: usize) -> u8 {
    debug_assert!(index < DIMENSION);
    let index = index * 4;

    value.shr(index).bitand(NYBBLE) as u8
}

#[derive(Default)]
struct NybbleRow(u32);

impl NybbleRow {
    pub fn set(&mut self, index: usize, value: u8) {
        self.0 &= !nybble_shift(NYBBLE as u8, index);
        self.0 |= nybble_shift(value, index);
    }

    pub fn get(&self, index: usize) -> u8 {
        nybble_unshift(self.0, index) 
    }
}

#[derive(Default)]
pub struct NybbleField([NybbleRow; DIMENSION]);

impl NybbleField {
    pub fn set(&mut self, Coord(col, row): Coord<usize>, value: u8) {
        self.0[row].set(col, value);
    }
    pub fn get(&self, Coord(col, row): Coord<usize>) -> u8 {
        self.0[row].get(col)
    }
}