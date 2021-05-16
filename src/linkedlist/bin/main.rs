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
                // TODO
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
                            return Some(box_node.value);
                        }
                    }
                },
                None => {
                    return None;
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
