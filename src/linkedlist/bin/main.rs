use std::fmt::Display;
use std::env::current_dir;

struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct List {
    head: Option<Box<Node>>
}

impl List {
    pub fn length(&self) -> usize {
        let mut count = 0;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            count = count + 1;
            current_node = node.next.as_ref().map(|node| &*node)
        }
        count
    }

    fn get_mut_node_at(&mut self, n: usize) -> Option<&mut Box<Node>> {
        let mut nth_node = self.head.as_mut();
        for _ in 0..n {
            nth_node = match nth_node {
                None => return None,
                Some(node) => node.next.as_mut().map(|node| &mut *node),
            }
        }
        nth_node
    }

    fn dump(&self) {
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            println!("{}", node.value);
            current_node = node.next.as_ref().map(|node| &*node)
        }
    }

    fn insert_at_last(&mut self, value: i32) {
        match self.head {
            Some(_) => {
                let len = self.length();
                let current_node = self.get_mut_node_at(len - 1);

                if let Some(node) = current_node {
                    node.next = Some(Box::new(Node {
                        value,
                        next: None
                    }))
                }
            },
            None => {
                self.head = Some(Box::new(Node {
                    value,
                    next: None
                }))
            }
        }
    }

    fn iter(&self) -> ListIterator {
        let x = self.head.as_ref();

        ListIterator {
            cursor: x
        }
    }
}

struct ListIterator<'a> {
    cursor: Option<&'a Box<Node>>
}

impl Iterator for ListIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_node) = self.cursor {
            if let Some(next_node) = current_node.next.as_ref() {
                self.cursor = Some(next_node);
            } else {
                self.cursor = None
            }
            Some(current_node.value)
        } else {
            None
        }
    }
}

fn main() {
    let mut list = List { head: None };
    list.insert_at_last(1);
    list.insert_at_last(2);
    list.insert_at_last(3);
    list.insert_at_last(4);
    list.insert_at_last(5);
    // list.dump();

    for x in list.iter() {
        dbg!(x);
    }
}
