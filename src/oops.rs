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

use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Oops {
    Message(String),
    RealError(Box<dyn std::error::Error>),
}

impl Display for Oops {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Oops::Message(s) => write!(f, "oops: {}", s)?,
            Oops::RealError(e) => e.fmt(f)?,
        }
        Ok(())
    }
}

impl<E> From<E> for Oops
where
    E: std::error::Error + 'static,
{
    fn from(error: E) -> Self {
        Oops::RealError(Box::new(error))
    }
}

#[macro_export]
macro_rules! oops {
    ($($e:expr),*) => {
        Oops::Message(format!($($e,)*))
    };
}
