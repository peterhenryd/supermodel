#![allow(dead_code)]
// TODO: unimplemented

use std::collections::VecDeque;

pub struct Decoder {
    pub rows: VecDeque<Row>,
}

pub struct Row {
    index: usize,
}

impl Decoder {
}

impl supermodel::dialect::decode::Decoder for Decoder {
}