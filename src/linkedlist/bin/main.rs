// use std::collections::LinkedList;

use std::fmt::Display;

struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct List {
    head: Option<Box<Node>>
}

impl List {
    // fn iter(&self) -> ListIterator {
    //     if let Some(node) = self.head.as_ref() {
    //         ListIterator {
    //             current_node: node
    //         }
    //     } else {
    //         None
    //     }
    // }
}

struct ListIterator<'a> {
    current_node: &'a Box<Node>
}

impl Iterator for ListIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let current_node = self.current_node.next.as_ref();
        if let Some(node) = current_node {
            self.current_node = node;
            Some(node.value)
        } else {
            None
        }
    }
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
}

#[derive(Debug)]
struct Fibonacci {
    a: i32,
    b: i32
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci { a: 0, b: 1}
    }
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.a;
        self.a = self.b;
        self.b += x;
        Some(x)
    }
}

fn main() {
    let mut list = List { head: None };
    list.insert_at_last(1);
    list.insert_at_last(2);
    list.insert_at_last(3);
    list.dump();

    // for x in list.iter() {
    //     dbg!(x);
    // }

    let mut x = Fibonacci::new();

    for _ in (0..10) {
        dbg!(x.next().unwrap());
    }
}
