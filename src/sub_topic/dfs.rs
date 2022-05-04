use std::fmt::Debug;

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

pub trait DFS<T> {
    fn dfs(&self) -> Vec<T>;
}

impl<T> DFS<T> for Tree<T> 
where 
    T: Copy + Debug,
{
    fn dfs(&self) -> Vec<T> {
        let mut res = Vec::new();
        let mut stack = vec![self];

        while stack.len()>0 {
            if let Some(cur) = stack.pop() {
                if let Some(val) = cur.value {
                    res.push(val);
                };

                if let Some(next) = cur.branch.as_ref() {
                    next.iter().rev().for_each(|v| stack.push(v));
                }
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

    let res = tree.dfs();
    println!("dfs: {res:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        main();
    }
}

