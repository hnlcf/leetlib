#[derive(Debug, Default)]
pub struct MyLinkedList {
    val: i32,
    next: Option<Box<MyLinkedList>>,
}

impl MyLinkedList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn new_full(val: i32, next: Option<Box<MyLinkedList>>) -> Self {
        Self { val, next }
    }

    pub fn get(&self, index: i32) -> i32 {
        let mut i = 0;
        let mut list = &self.next;
        while let Some(entry) = list {
            if i == index {
                return entry.val;
            }

            i += 1;
            list = &entry.next;
        }
        -1
    }

    pub fn add_at_head(&mut self, val: i32) {
        let node = MyLinkedList::new_full(val, self.next.take());
        self.next = Some(Box::new(node));
    }

    pub fn add_at_tail(&mut self, val: i32) {
        let node = MyLinkedList::new_full(val, None);

        let mut list = &mut self.next;
        while let Some(entry) = list {
            list = &mut entry.next;
        }

        *list = Some(Box::new(node));
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if index <= 0 {
            self.add_at_head(val);
            return;
        }

        let mut i = 0;
        let mut list = &mut self.next;
        while let Some(entry) = list {
            if i + 1 == index {
                let node = MyLinkedList::new_full(val, entry.next.take());
                entry.next = Some(Box::new(node));

                break;
            }

            i += 1;
            list = &mut entry.next;
        }
    }

    pub fn delete_at_index(&mut self, index: i32) {
        self.delete_at_index_impl(index);
    }

    pub fn delete_at_index_impl(&mut self, index: i32) -> Option<i32> {
        let mut i = 0;
        let mut list = self;
        while let Some(entry) = list.next.take() {
            if i == index {
                list.next = entry.next;
                return Some(entry.val);
            }

            i += 1;
            list.next = Some(entry);
            list = list.next.as_mut()?;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut linked_list = MyLinkedList::new();
        linked_list.add_at_head(1);
        linked_list.add_at_tail(3);
        println!("{:#?}", linked_list);

        linked_list.add_at_index(1, 2);
        println!("{:#?}", linked_list);

        assert_eq!(linked_list.get(1), 2);
        linked_list.delete_at_index_impl(1);
        println!("{:#?}", linked_list);

        assert_eq!(linked_list.get(1), 3);
    }

    #[test]
    fn ex2() {
        let mut linked_list = MyLinkedList::new();

        linked_list.add_at_index(0, 10);
        linked_list.add_at_index(0, 20);
        linked_list.add_at_index(1, 30);
        println!("{:#?}", linked_list); // [head;0] -> [20] -> [30] ->[10]

        assert_eq!(linked_list.get(0), 20);
    }
}
