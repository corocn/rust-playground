// use std::collections::LinkedList;

struct Node {
    value: i32,
    next: Option<Box<Node>>
}

struct List {
    head: Option<Node>
}

impl List {
    pub fn length(&self) -> usize {
        let mut count = 0;
        let mut current_node = self.head.as_ref();
        while let Some(node) = current_node {
            count = count + 1;
            current_node = node.next.as_ref().map(|node| &**node)
        }
        count
    }

    fn get_mut_node_at(&mut self, n: usize) -> Option<&mut Node> {
        let mut nth_node = self.head.as_mut();
        for _ in 0..n {
            nth_node = match nth_node {
                None => return None,
                Some(node) => node.next.as_mut().map(|node| &mut **node),
            }
        }
        nth_node
    }

    fn insert_at_last(&mut self, value: i32) {
        match self.head {
            Some(_) => {
                let len = self.length();
                let mut current_node = self.get_mut_node_at(len - 1);

                if let Some(node) = current_node {
                    node.next = Some(Box::new(Node {
                        value,
                        next: None
                    }))
                }
            },
            None => {
                self.head = Some(Node {
                    value,
                    next: None
                })
            }
        }
    }
}

fn main() {
    let mut list = List { head: None };
    println!("{}", list.length());
    list.insert_at_last(1);
    println!("{}", list.length());
    list.insert_at_last(2);
    println!("{}", list.length());
    list.insert_at_last(3);
    println!("{}", list.length());

    // let v = list.last();
    // println!("{}", v.unwrap())

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
