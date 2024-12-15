use crate::util::point::*;

use std::borrow::Borrow;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    pub data: Vec<T>,
}

impl Grid<u8> {
    pub fn parse(input: &[String]) -> Self {
        let width = input[0].len();
        let height = input.len();
        let mut data = Vec::with_capacity(width * height);
        input
            .iter()
            .for_each(|line| data.extend_from_slice(line.trim().as_bytes()));
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn parse_str(input: &str) -> Self {
        Grid::parse(
            &input
                .lines()
                .map(|l| l.trim().to_owned())
                .collect::<Vec<_>>(),
        )
    }
}

impl<T: PartialEq> Grid<T> {
    #[inline]
    pub fn contains<P>(&self, point: P) -> bool
    where
        P: Borrow<Point>,
    {
        let point = point.borrow();
        point.x >= 0 && point.x < self.width as i64 && point.y >= 0 && point.y < self.height as i64
    }

    pub fn find<U>(&self, needle: U) -> Option<Point>
    where
        T: Borrow<U>,
        U: PartialEq,
    {
        self.data
            .iter()
            .position(|x| x.borrow() == &needle)
            .map(|i| Point::new((i % self.width) as i64, (i / self.width) as i64))
    }
}

impl<T> Index<Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Point) -> &Self::Output {
        &self[&index]
    }
}

impl<T> IndexMut<Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self[&index]
    }
}

impl<T> Index<&Point> for Grid<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &Point) -> &Self::Output {
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl<T> IndexMut<&Point> for Grid<T> {
    #[inline]
    fn index_mut(&mut self, index: &Point) -> &mut Self::Output {
        &mut self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.data[y * self.width + x] as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> GridIter<T> {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }

    pub fn iter_mut(&mut self) -> GridIterMut<T> {
        GridIterMut {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = (Point, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            return None;
        }
        let point = Point::new(self.x as i64, self.y as i64);

        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        Some((point, &self.grid[&point]))
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = (Point, &'a T);
    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct GridIterMut<'a, T> {
    grid: &'a mut Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIterMut<'a, T> {
    type Item = (Point, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.height {
            return None;
        }
        let point = Point::new(self.x as i64, self.y as i64);
        self.x += 1;
        if self.x >= self.grid.width {
            self.x = 0;
            self.y += 1;
        }

        let value = unsafe {
            let ptr = self.grid.data.as_mut_ptr();
            &mut *ptr.add(point.y as usize * self.grid.width + point.x as usize)
        };

        Some((point, value))
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = (Point, &'a mut T);
    type IntoIter = GridIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
