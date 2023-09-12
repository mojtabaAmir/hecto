use crate::Position;
use crate::Row;
use std::fs;
use std::io::{Error, Write};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
    dirty: bool,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_owned()),
            dirty: false,
        })
    }
    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    fn insert_newline(&mut self, at: &Position) {
        if at.y == self.len() {
            self.rows.push(Row::default());
            return
        }
        let new_row = self.rows.get_mut(at.y).unwrap().split(at.x);
        #[allow(clippy::arithmetic_side_effects)]
        self.rows.insert(at.y + 1, new_row);
    } 
    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        }
        self.dirty = true;
        if c == '\n' {
            self.insert_newline(at);
            return;
        } 
        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }
    #[allow(clippy::arithmetic_side_effects)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();
        if at.y >= len {
            return;
        }
        self.dirty = true;
        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.delete(at.x);
        } 
    }
    pub fn cut_row(&mut self, at: &Position) -> Option<String> {
        if at.y >= self.len() {
            return None;
        }
        let is_cut_row = self.rows.remove(at.y);
        self.dirty = true;
        Some(is_cut_row.to_string().clone())
    }
    pub fn save(&mut self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;
            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }
        Ok(())
    }
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
    pub fn find(&self, query: &str, after: &Position) -> Option<Position> {
        let mut x = after.x;
        for (y, row) in self.rows.iter().enumerate().skip(after.y) {
            if let Some(x) = row.find(query, x) {
                return Some(Position { x, y });
            }
            x = 0;
        }
        None
    } 
}
