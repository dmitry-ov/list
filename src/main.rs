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
}

#[derive(Debug)]
struct LinkedListIterator {
    current_iter_elem: Option<Rc<RefCell<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        Self {
            head: None,
            last: None,
        }
    }

    fn push_front(&mut self, value: i32) {
        if let Some(head) = &self.head {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: value,
                next: Some(Rc::clone(head)),
            })));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node {
                data: value,
                next: None,
            })));
        }
    }

    fn insert_after_n(&mut self, value: i32, index: i32) -> Result<(), String> {
        if self.size() < index {
            return Err(format!("no item with index {index}"));
        }

        if self.size() == index {
            self.push_back(value);
            return Ok(());
        }

        let mut current = self.head.clone();
        for _ in 0..index {
            if let Some(node) = current {
                current = node.borrow().next.clone();
            }
        }

        let Some(cur) = &current else {
            return Err(format!("can't insert after item with index {index}"));
        };

        let tmp = cur.clone();
        let mut ref_mut = tmp.borrow_mut();
        let tmp_next = ref_mut.next.clone();

        ref_mut.next = Some(Rc::new(RefCell::new(Node {
            data: value,
            next: tmp_next,
        })));
        Ok(())
    }

    pub fn size(&self) -> i32 {
        if let Some(head) = &self.head {
            let mut count = 1;
            let mut current = head.clone();
            while let Some(node) = current.clone().borrow().next.clone() {
                current = node.clone();
                count += 1;
            }
            return count;
        }
        0
    }

    pub fn push_back(&mut self, value: i32) {
        let new_rc = Rc::new(RefCell::new(Node {
            data: value,
            next: None,
        }));
        if let Some(last) = &self.last {
            last.borrow_mut().next = Some(new_rc.clone());
            self.last = Some(new_rc.clone());
        } else {
            self.head = Some(new_rc.clone());
            self.last = self.head.clone();
        }
    }

    fn split_at_index(self, index: i32) -> (LinkedList, LinkedList) {
        let mut first_list = LinkedList::new();
        let mut second_list = LinkedList::new();

        let mut current_node = self.head;
        let mut current_index = 0;

        while let Some(node) = current_node {
            let borrowed_node = node.borrow();
            let node_data = borrowed_node.data;

            if current_index < index {
                first_list.push_back(node_data);
            } else {
                second_list.push_back(node_data);
            }

            current_node = borrowed_node.next.clone();
            current_index += 1;
        }
        (first_list, second_list)
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

    fn iter(&self) -> LinkedListIterator {
        LinkedListIterator {
            current_iter_elem: self.head.clone(),
        }
    }
}

impl Iterator for LinkedListIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_iter_elem.as_ref()?;

        if let Some(node) = self.current_iter_elem.clone() {
            self.current_iter_elem = node.borrow().next.clone();
            Some(node.borrow().data)
        } else {
            None
        }
    }
}

fn main() {
    //добавлять элемент в начало,
    let mut list = LinkedList::new();
    list.push_front(3);
    list.push_front(2);
    list.push_front(1);

    for x in list.iter() {
        println!("{:?}", x);
    }

    //добавлять элемент в конец,
    let mut list2 = LinkedList::new();
    list2.push_back(4);
    list2.push_back(5);
    list2.push_back(6);

    for x in list2.iter() {
        println!("{:?}", x);
    }

    // добавлять элемент после N-го
    let mut list3 = LinkedList::new();
    list3.push_back(7);
    list3.push_back(9);
    let _ = list3.insert_after_n(8, 1);

    for x in list3.iter() {
        println!("{:?}", x);
    }

    // Разделяться на два списка: от начального элемента до- (N-1)-го и от (N-1)-го до последнего.
    let mut list_split = LinkedList::new();
    list_split.push_back(10);
    list_split.push_back(11);
    list_split.push_back(12);
    list_split.push_back(13);
    list_split.push_back(14);
    list_split.push_back(15);

    let (first_part, second_part) = list_split.split_at_index(2);
    for a in first_part.iter() {
        println!("{:?}", a);
    } // 10 11

    for b in second_part.iter() {
        println!("{:?}", b);
    } // 12 13 14 15

    // Предоставлять возможность изменять элементы списка
    let mut list_edit = LinkedList::new();
    list_edit.push_back(50);
    list_edit.replace_n_item_with_value(0, 100);
    println!("{:?}", list_edit);
}

#[cfg(test)]
mod tests {
    use crate::LinkedList;

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push_back(1);
        let mut result = Vec::new();
        for x in list.iter() {
            result.push(x);
        }
        assert_eq!(vec![1], result);
    }

    #[test]
    fn push_front() {
        let mut list = LinkedList::new();
        list.push_front(3);
        list.push_front(2);
        list.push_front(1);
        let mut result = Vec::new();
        for x in list.iter() {
            result.push(x);
        }
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn push_back_to_empty() {
        let mut list = LinkedList::new();
        list.push_back(1);
        assert_eq!(1, list.head.unwrap().borrow().data);
        assert_eq!(1, list.last.unwrap().borrow().data);
    }

    #[test]
    fn push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        assert_eq!(4, list.size());
        assert_eq!(1, list.head.unwrap().borrow().data);
        assert_eq!(4, list.last.unwrap().borrow().data);
    }

    #[test]
    fn size() {
        assert_eq!(0, LinkedList::new().size());

        let mut list_one = LinkedList::new();
        list_one.push_back(1);
        assert_eq!(1, list_one.size());

        let mut list_five = LinkedList::new();
        list_five.push_back(1);
        list_five.push_back(2);
        list_five.push_back(3);
        list_five.push_back(4);
        list_five.push_back(5);
        assert_eq!(5, list_five.size());
    }

    #[test]
    fn insert_after_n() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(4);

        let result = list.insert_after_n(7, 1);
        assert!(result.is_ok());
        assert_eq!(4, list.size());
    }

    #[test]
    fn insert_after_n_with_empty() {
        let result_for_empty = LinkedList::new().insert_after_n(7, 55);
        assert_eq!("no item with index 55", result_for_empty.err().unwrap());

        let result = LinkedList::new().insert_after_n(7, 1);
        assert!(result.is_err());

        let result = LinkedList::new().insert_after_n(7, 0);
        assert!(result.is_ok());
    }

    #[test]
    fn add_after_last() {
        let mut l = LinkedList::new();
        l.push_back(1);
        l.push_back(2);
        let result = l.insert_after_n(4, 2);
        assert!(result.is_ok());
        assert_eq!(3, l.size());
    }

    #[test]
    fn split_after_n() {
        let mut list_split = LinkedList::new();
        list_split.push_back(10);
        list_split.push_back(11);
        list_split.push_back(12);
        list_split.push_back(13);
        list_split.push_back(14);
        list_split.push_back(15);

        let (fst, snd) = list_split.split_at_index(2);
        assert_eq!(2, fst.size());
        assert_eq!(4, snd.size());
    }

    #[test]
    fn replace_n_item_with_value() {
        let mut list_edit = LinkedList::new();
        list_edit.push_back(50);
        list_edit.replace_n_item_with_value(0, 100);
        println!("{:?}", list_edit);
    }
}
