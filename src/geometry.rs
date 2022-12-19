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
use std::ops::Add;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

pub struct Neighbors<'a> {
    p: &'a Point3,
    iter: std::slice::Iter<'static, Vector3>,
}

impl<'a> Iterator for Neighbors<'a> {
    type Item = Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.iter.next() {
            Some(*self.p + *v)
        } else {
            None
        }
    }
}

impl Point3 {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Point3 { x, y, z }
    }

    pub fn neighbors(&self) -> Neighbors {
        const NEIGHBOR_VECTORS: &[Vector3] = &[
            Vector3::new(-1, 0, 0),
            Vector3::new(1, 0, 0),
            Vector3::new(0, -1, 0),
            Vector3::new(0, 1, 0),
            Vector3::new(0, 0, -1),
            Vector3::new(0, 0, 1),
        ];

        Neighbors {
            p: self,
            iter: NEIGHBOR_VECTORS.iter(),
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
    pub fn contains(&self, p: &Point3) -> bool {
        p.x >= self.min.x
            && p.x <= self.max.x
            && p.y >= self.min.y
            && p.y <= self.max.y
            && p.z >= self.min.z
            && p.z <= self.max.z
    }

    pub fn from_point3s<I>(i: I) -> Self
    where
        I: IntoIterator,
        I::Item: Borrow<Point3>,
    {
        i.into_iter()
            .fold(Self::new_uninitialized(), |b, p| Bounds3 {
                min: Point3::new(
                    std::cmp::min(b.min.x, p.borrow().x - 1),
                    std::cmp::min(b.min.y, p.borrow().y - 1),
                    std::cmp::min(b.min.z, p.borrow().z - 1),
                ),
                max: Point3::new(
                    std::cmp::max(b.max.x, p.borrow().x + 1),
                    std::cmp::max(b.max.y, p.borrow().y + 1),
                    std::cmp::max(b.max.z, p.borrow().z + 1),
                ),
            })
    }

    fn new_uninitialized() -> Self {
        Bounds3 {
            min: Point3::new(i32::MAX, i32::MAX, i32::MAX),
            max: Point3::new(i32::MIN, i32::MIN, i32::MIN),
        }
    }
}
