//  Copyright 2022 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point2 {
    pub x: i32,
    pub y: i32,
}

impl Point2 {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Point2 { x, y }
    }

    #[must_use]
    pub fn neighbors(&self) -> Neighbors2 {
        const NEIGHBOR_VECTORS: &[Vector2] = &[
            Vector2::new(-1, 0),
            Vector2::new(1, 0),
            Vector2::new(0, -1),
            Vector2::new(0, 1),
        ];

        Neighbors2 {
            p: self,
            iter: NEIGHBOR_VECTORS.iter(),
        }
    }
}

pub struct Neighbors2<'a> {
    p: &'a Point2,
    iter: std::slice::Iter<'static, Vector2>,
}

impl<'a> Iterator for Neighbors2<'a> {
    type Item = Point2;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.iter.next() {
            Some(*self.p + *v)
        } else {
            None
        }
    }
}

impl Add<Vector2> for Point2 {
    type Output = Point2;
    fn add(self, rhs: Vector2) -> Self::Output {
        Point2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Vector2> for Point2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point2 {
    type Output = Vector2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32,
}

impl Vector2 {
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// TODO: Maybe this should be a rectangle class?
#[derive(Debug)]
pub struct Bounds2 {
    pub min: Point2,
    pub max: Point2,
}

impl Bounds2 {
    #[must_use]
    pub fn contains(&self, p: &Point2) -> bool {
        p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y
    }

    // TODO: What numeric type should this use?
    #[must_use]
    pub fn height(&self) -> i32 {
        self.max.x - self.min.x
    }

    // TODO: What numeric type should this use?
    #[must_use]
    pub fn width(&self) -> i32 {
        self.max.y - self.min.y
    }

    #[must_use]
    pub fn from_points<I>(i: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point2>,
    {
        i.into_iter().fold(Self::new_uninitialized(), |b, p| Self {
            min: Point2::new(
                std::cmp::min(b.min.x, p.borrow().x),
                std::cmp::min(b.min.y, p.borrow().y),
            ),
            max: Point2::new(
                std::cmp::max(b.max.x, p.borrow().x),
                std::cmp::max(b.max.y, p.borrow().y),
            ),
        })
    }

    #[must_use]
    fn new_uninitialized() -> Self {
        Bounds2 {
            min: Point2::new(i32::MAX, i32::MAX),
            max: Point2::new(i32::MIN, i32::MIN),
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point3 {
    #[must_use]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }

    #[must_use]
    pub fn neighbors(&self) -> Neighbors3 {
        const NEIGHBOR_VECTORS: &[Vector3] = &[
            Vector3::new(-1, 0, 0),
            Vector3::new(1, 0, 0),
            Vector3::new(0, -1, 0),
            Vector3::new(0, 1, 0),
            Vector3::new(0, 0, -1),
            Vector3::new(0, 0, 1),
        ];

        Neighbors3 {
            p: self,
            iter: NEIGHBOR_VECTORS.iter(),
        }
    }
}

pub struct Neighbors3<'a> {
    p: &'a Point3,
    iter: std::slice::Iter<'static, Vector3>,
}

impl<'a> Iterator for Neighbors3<'a> {
    type Item = Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.iter.next() {
            Some(*self.p + *v)
        } else {
            None
        }
    }
}

impl Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Point3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Vector3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector3 {
    const fn new(x: i32, y: i32, z: i32) -> Self {
        Vector3 { x, y, z }
    }
}

// TODO: Maybe this should be a cube class?
pub struct Bounds3 {
    pub min: Point3,
    pub max: Point3,
}

impl Bounds3 {
    #[must_use]
    pub fn contains(&self, p: &Point3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    #[must_use]
    pub fn outset(&self, n: i32) -> Self {
        Bounds3 {
            min: Point3::new(self.min.x - n, self.min.y - n, self.min.z - n),
            max: Point3::new(self.max.x + n, self.max.x + n, self.max.z + n),
        }
    }

    #[must_use]
    pub fn from_points<I>(i: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point3>,
    {
        i.into_iter().fold(Self::new_uninitialized(), |b, p| Self {
            min: Point3::new(
                std::cmp::min(b.min.x, p.borrow().x),
                std::cmp::min(b.min.y, p.borrow().y),
                std::cmp::min(b.min.z, p.borrow().z),
            ),
            max: Point3::new(
                std::cmp::max(b.max.x, p.borrow().x),
                std::cmp::max(b.max.y, p.borrow().y),
                std::cmp::max(b.max.z, p.borrow().z),
            ),
        })
    }

    #[must_use]
    fn new_uninitialized() -> Self {
        Self {
            min: Point3::new(i32::MAX, i32::MAX, i32::MAX),
            max: Point3::new(i32::MIN, i32::MIN, i32::MIN),
        }
    }
}
