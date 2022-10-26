use core::fmt;

#[derive(Debug, Default)]
pub struct SingleLinkedList<T> {
    val: T,
    next: Option<Box<SingleLinkedList<T>>>,
}

impl<T> SingleLinkedList<T>
where
    T: fmt::Debug,
{
    pub fn new(val: T, next: Option<Box<SingleLinkedList<T>>>) -> Self {
        Self { val, next }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut i = 0;
        let mut list = &self.next;
        while let Some(entry) = list {
            if i == index {
                return Some(&entry.val);
            }

            i += 1;
            list = &entry.next;
        }
        None
    }

    pub fn add_at_head(&mut self, val: T) {
        let node = SingleLinkedList::new(val, self.next.take());
        self.next = Some(Box::new(node));
    }

    pub fn add_at_tail(&mut self, val: T) {
        let node = SingleLinkedList::new(val, None);

        let mut list = &mut self.next;
        while let Some(entry) = list {
            list = &mut entry.next;
        }

        *list = Some(Box::new(node));
    }

    pub fn add_at_index(&mut self, index: usize, val: T) {
        if index == 0 {
            self.add_at_head(val);
            return;
        }

        let mut i = 0;
        let mut list = &mut self.next;
        while let Some(entry) = list {
            if i + 1 == index {
                let node = SingleLinkedList::new(val, entry.next.take());
                entry.next = Some(Box::new(node));

                break;
            }

            i += 1;
            list = &mut entry.next;
        }
    }

    pub fn delete_at_index(&mut self, index: usize) {
        self.delete_at_index_impl(index);
    }

    pub fn delete_at_index_impl(&mut self, index: usize) -> Option<T> {
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

impl<T: fmt::Debug> fmt::Display for SingleLinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut list = &self.next;

        write!(f, "[ head ] -> ")?;
        while let Some(entry) = list {
            write!(f, "[ {:?} ] -> ", entry.val)?;

            list = &entry.next;
        }
        writeln!(f, "None")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let mut linked_list = SingleLinkedList::default();
        linked_list.add_at_head(1);
        linked_list.add_at_tail(3);
        println!("{}", linked_list);

        linked_list.add_at_index(1, 2);
        println!("{}", linked_list);

        assert_eq!(linked_list.get(1), Some(&2));
        linked_list.delete_at_index_impl(1);
        println!("{}", linked_list);

        assert_eq!(linked_list.get(1), Some(&3));
    }

    #[test]
    fn ex2() {
        let mut linked_list = SingleLinkedList::default();

        linked_list.add_at_index(0, 10);
        linked_list.add_at_index(0, 20);
        linked_list.add_at_index(1, 30);
        println!("{}", linked_list);

        assert_eq!(linked_list.get(0), Some(&20));
    }
}
