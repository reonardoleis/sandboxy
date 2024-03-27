use std::fmt::{Debug, Formatter, Result};

use crate::materials::material::Material;

pub struct Grid {
    width: u8,
    height: u8,
    mat: Vec<Vec<Option<Box<dyn Material>>>>,
}

impl Grid {
    pub fn new(width: u8, height: u8) -> Self {
        let mut mat: Vec<Vec<Option<Box<dyn Material>>>> = Vec::new();

        for _ in 0..width {
            let mut row = Vec::new();
            for _ in 0..height {
                row.push(None);
            }
            mat.push(row);
        }

        Grid { width, height, mat }
    }

    pub fn add(&mut self, x: usize, y: usize, material: Box<dyn Material>) {
        if x >= self.width as usize || y >= self.height as usize {
            return;
        }

        self.mat[x][y] = Some(material);
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut mat_str = String::new();

        for row in &self.mat {
            for cell in row {
                mat_str.push(match cell {
                    Some(m) => m.as_ref().to_char(),
                    None => '0',
                });
            }
            mat_str.push('\n');
        }

        write!(
            f,
            "Grid {{ \nwidth: {},\nheight: {}\nmat: \n{} }}",
            self.width, self.height, mat_str,
        )
    }
}
