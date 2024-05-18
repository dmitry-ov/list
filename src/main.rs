use std::cell::RefCell;
use std::iter::Iterator;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    data: i32,
    next: Option<Rc<RefCell<Node>>>,
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Rc<RefCell<Node>>>,
    last: Option<Rc<RefCell<Node>>>,
    current_iter_elem: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            last: None,
            current_iter_elem: None,
        }
    }

    fn add_before_head(&mut self, value: i32) {
        if self.head.is_none() {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: value,
                next: None,
            })));
        } else {
            let copy_of_head = Rc::clone(self.head.as_ref().unwrap());
            self.head = Some(Rc::new(RefCell::new(Node {
                data: value,
                next: Some(Rc::clone(&copy_of_head)),
            })));
        }
        self.current_iter_elem = self.head.clone();
    }

    fn add_after_n(&mut self, value: i32, mut n: i32) {
        let mut current = self.head.clone();

        while n == 0 {
            current = current.unwrap().borrow().next.clone();
            n -= 1;
        }

        let tmp = current.clone().unwrap();
        let mut ref_mut = tmp.borrow_mut();
        let tmp_next = ref_mut.next.clone();

        ref_mut.next = Some(Rc::new(RefCell::new(Node {
            data: value,
            next: tmp_next,
        })))
    }

    fn add_after_tail(&mut self, value: i32) {
        if self.head.is_none() {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: value,
                next: None,
            })));
            self.current_iter_elem = self.head.clone();
            self.last = self.head.clone();
        } else {
            let mut current = self.head.clone();
            while let Some(node) = current {
                current = node.borrow().next.clone();
                self.last = Some(node.clone());
            }
            self.last.as_ref().unwrap().borrow_mut().next = Some(Rc::new(RefCell::new(Node {
                data: value,
                next: None,
            })));
        }
    }

    fn split_after_n(&self, mut n: i32) -> (LinkedList, LinkedList) {
        let mut fst = LinkedList::new();
        let mut snd = LinkedList::new();

        let mut current = self.head.clone();
        while let Some(node) = current {
            let x = node.borrow();
            if n != 0 {
                n -= 1;
                fst.add_after_tail(x.data);
            } else {
                snd.add_after_tail(x.data);
            }
            current = x.next.clone();
        }
        (fst, snd)
    }

    fn replace_n_item_with_value(&mut self, mut n: i32, value: i32) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if n != 0 {
                n -= 1;
                current = node.borrow().next.clone();
            } else {
                node.borrow_mut().data = value;
                break;
            }
        }
    }
}

impl Iterator for LinkedList {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_iter_elem.as_ref()?;

        let rc = self.current_iter_elem.clone().unwrap();
        let x = rc.borrow();
        let value = x.data;
        if x.next.is_some() {
            self.current_iter_elem = Some(Rc::clone(x.next.as_ref().unwrap()));
        } else {
            self.current_iter_elem = None;
        }
        Some(value)
    }
}

fn main() {
    //добавлять элемент в начало,
    let mut list = LinkedList::new();
    list.add_before_head(3);
    list.add_before_head(2);
    list.add_before_head(1);

    for x in list {
        println!("{:?}", x);
    }

    //добавлять элемент в конец,
    let mut list2 = LinkedList::new();
    list2.add_after_tail(4);
    list2.add_after_tail(5);
    list2.add_after_tail(6);

    for x in list2 {
        println!("{:?}", x);
    }

    // добавлять элемент после N-го
    let mut list3 = LinkedList::new();
    list3.add_after_tail(7);
    list3.add_after_tail(9);
    list3.add_after_n(8, 1);

    for x in list3 {
        println!("{:?}", x);
    }

    // Разделяться на два списка: от начального элемента до- (N-1)-го и от (N-1)-го до последнего.
    let mut list_split = LinkedList::new();
    list_split.add_after_tail(10);
    list_split.add_after_tail(11);
    list_split.add_after_tail(12);
    list_split.add_after_tail(13);
    list_split.add_after_tail(14);
    list_split.add_after_tail(15);

    let (first_part, second_part) = list_split.split_after_n(2);
    for a in first_part {
        println!("{:?}", a);
    } // 10 11

    for b in second_part {
        println!("{:?}", b);
    } // 12 13 14 15

    // Предоставлять возможность изменять элементы списка
    let mut list_edit = LinkedList::new();
    list_edit.add_after_tail(50);
    list_edit.replace_n_item_with_value(0, 100);
    println!("{:?}", list_edit);
}
