use std::fmt::Debug;
use std::collections::VecDeque;

#[derive(Debug)]
struct Tree<T> {
    value: Option<T>,
    branch: Option<Box<Vec<Tree<T>>>>
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            value: None,
            branch: None
        }
    }

    pub fn set(&mut self, value: T) {
        self.value = Some(value);
    }

    pub fn growth(&mut self, leaf_value: T) {
        let leaf = Tree{
            value: Some(leaf_value),
            branch: None
        };

        if let Some(ref mut branch) = self.branch {
            branch.push(leaf);
        } else {
            self.branch = Some(Box::new(vec![leaf]));
        }
    }

    pub fn point_branch_by_index(&mut self, index: usize) -> Result<&mut Tree<T>,String> {
        if let Some(branch) = self.branch.as_mut() {
            if index<branch.len() {
                Ok(&mut branch[index])
            } else {
                Err(format!("Index out of bound. This branch only has {} leaf.",branch.len()))
            }
        } else {
            Err(format!("Index out of bound. This branch only has {} leaf.", 0))
        }
    }
}

pub trait BFS<T> {
    fn bfs(&self) -> Vec<T>;
}

impl<T> BFS<T> for Tree<T> 
where 
    T: Copy + Debug,
{
    fn bfs(&self) -> Vec<T> {
        let mut res = Vec::new();
        let mut queue = VecDeque::from([self]);

        while queue.len()>0 {
            let cur = queue.pop_front().unwrap();
            if let Some(val) = cur.value {
                res.push(val);
            }
            if let Some(next) = cur.branch.as_ref() {
                next.iter().for_each(|v| queue.push_back(v));
            }
        }
        res
    }
}


fn main() {
    /*
    1 -> [2, 5, 6]
        2 -> [3, 4]
        5 -> []
        6 -> []
            3 -> []
            4 -> []
    */

    let mut tree = Tree::new();
    tree.set(1);
    tree.growth(2);
    tree.growth(5);
    tree.growth(6);

    let branch = tree.point_branch_by_index(0).unwrap();
    branch.growth(3);
    branch.growth(4);

    // println!("{tree:#?}");

    let res = tree.bfs();
    println!("bfs: {res:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        main();
    }
}

