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

pub struct Segmenter<I: Iterator, P> {
    iter: I,
    predicate: P,
}

impl<I: Iterator, P> Iterator for Segmenter<I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut items: Vec<I::Item> = Vec::new();
        loop {
            if let Some(item) = self.iter.next() {
                if (self.predicate)(&item) {
                    if items.is_empty() {
                        continue;
                    }
                    return Some(items);
                }
                items.push(item);
            } else if items.is_empty() {
                return None;
            } else {
                return Some(items);
            }
        }
    }
}

pub trait IterTools {
    fn segment<P>(self, predicate: P) -> Segmenter<Self, P>
    where
        Self: Iterator + Sized,
        P: FnMut(&Self::Item) -> bool;
}

impl<T: Iterator> IterTools for T {
    fn segment<P>(self, predicate: P) -> Segmenter<T, P>
    where
        P: FnMut(&T::Item) -> bool,
    {
        Segmenter {
            iter: self,
            predicate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let v: Vec<u32> = vec![];
        assert_eq!(None, v.iter().segment(|x| **x == 0).next());
    }

    #[test]
    fn one_element() {
        let v: Vec<u32> = vec![1];
        let mut group_iter = v.iter().segment(|x| **x == 0);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[1]);
        assert_eq!(None, group_iter.next());
    }

    #[test]
    fn one_segment_two_elements() {
        let v: Vec<u32> = vec![1, 2];
        let mut group_iter = v.iter().segment(|x| **x == 0);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[1, 2]);
        assert_eq!(None, group_iter.next());
    }

    #[test]
    fn two_segments() {
        let v: Vec<u32> = vec![1, 0, 2];
        let mut group_iter = v.iter().segment(|x| **x == 0);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[1]);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[2]);
        assert_eq!(None, group_iter.next());
    }

    #[test]
    fn consecutive_segment_markers() {
        let v: Vec<u32> = vec![1, 0, 0, 2];
        let mut group_iter = v.iter().segment(|x| **x == 0);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[1]);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[2]);
        assert_eq!(None, group_iter.next());
    }

    #[test]
    fn segment_marker_at_first() {
        let v: Vec<u32> = vec![0, 1];
        let mut group_iter = v.iter().segment(|x| **x == 0);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[1]);
        assert_eq!(None, group_iter.next());
    }

    #[test]
    fn segment_marker_at_last() {
        let v: Vec<u32> = vec![1, 0];
        let mut group_iter = v.iter().segment(|x| **x == 0);
        let group = group_iter.next().unwrap();
        assert_eq!(group.into_iter().copied().collect::<Vec<_>>(), &[1]);
        assert_eq!(None, group_iter.next());
    }
}
