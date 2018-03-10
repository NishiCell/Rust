extern crate rand;
use rand::distributions;
use rand::distributions::IndependentSample;

use std::f32;

pub struct Tetris {
	pub block: Block,
	pub field: [[Color; 10];20],
}

pub struct Block {
	pub color: Color,
	pub blocks: Vec<(i32, i32)>,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Color {
	Black,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White
}

// ブロック操作コマンド
#[derive(PartialEq)]
pub enum Control {
	Down,
	Left,
	Tighe,
	Rotate
}

// 振ってくるブロックの色
const COLORS: &'static [Color] = &[
	Color::Red,
	Color::Green,
	Color::Blue,
	Color::Yellow,
	Color::Cyan,
	Color::Magenta,
];

// 振ってくるブロックの形
const BLOCKS: &'static [&'static [(i32,i32)]] =&[
	&[(0,0),(0,1),(1,0),(1,1)],
	&[(0,0),(0,1),(0,2),(1,1),(2,1)],
	&[(0,0),(0,1),(0,2),(0,3)],
	&[(0,0),(0,1),(0,2),(0,3),(1,3)],
	&[(0,0),(0,1),(0,2),(0,3),(1,0)],
	&[(0,0),(0,1),(1,1),(1,2)],
	&[(1,0),(1,1),(0,1),(0,2)],
	&[(0,1),(0,1),(0,2),(1,1)],
]





