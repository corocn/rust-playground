// use std::collections::LinkedList;

struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct List {
    head: Option<Box<Node>>
}

impl List {
    pub fn push_back(&mut self, value: i32) {
        match self.head {
            Some(_) => {
                let node: &Option<Box<Node>> = self.last_node();
                match node {
                    Some(mut box_node) => {
                        let mut n = box_node;
                        n.next = Some(Box::new(Node {
                            value,
                            next: None
                        }))
                    },
                    None => {}
                }
            },
            None => {
                self.head = Some(Box::new(Node {
                    value,
                    next: None
                }))
            },
        }
    }

    pub fn last(&self) -> Option<i32> {
        let node: &Option<Box<Node>> = self.last_node();

        match node {
            Some(box_node) => { Some(box_node.value) },
            None => { None }
        }
    }

    pub fn last_node(&self) -> &Option<Box<Node>> {
        let mut target: &Option<Box<Node>> = &self.head;

        loop {
            match target {
                Some(box_node) => {
                    let next: &Option<Box<Node>> = &box_node.next;

                    match next {
                        Some(_) => {
                            target = next;
                        },
                        None => {
                            return target;
                        }
                    }
                },
                None => {
                    return target;
                }
            }
        }
    }
}

fn main() {
    let mut list = List { head: None };
    list.push_back(1);
    // list.push_back(2);
    // list.push_back(3);

    let v = list.last();
    println!("{}", v.unwrap())


    // let mut list = LinkedList::new();

    // list.push_back(1);
    // list.push_back(2);
    // list.push_back(3);
    //
    // dbg!(&list);
    //
    // assert_eq!(list.pop_front(), Some(1));
    // assert_eq!(list.pop_front(), Some(2));
    // assert_eq!(list.pop_front(), Some(3));
}
