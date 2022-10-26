use core::fmt;

pub type DoubleNodePtr<T> = *mut DoubleNode<T>;

#[derive(Debug, Default)]
pub struct DoubleNode<T> {
    data: Option<T>,
    succ: Option<Box<DoubleNode<T>>>,
    prev: Option<*mut DoubleNode<T>>,
}

#[derive(Debug, Default)]
pub struct DoubleList<T>
where
    T: fmt::Debug + Default,
{
    head: DoubleNode<T>,
    len: usize,
}

impl<T> DoubleList<T>
where
    T: fmt::Debug + Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert_head(&mut self, elem: T) -> DoubleNodePtr<T> {
        self.len += 1;

        let mut new = Box::new(DoubleNode {
            data: Some(elem),
            succ: None,
            prev: Some(&mut self.head as *mut DoubleNode<T>),
        });
        let p_new = new.as_mut() as *mut DoubleNode<T>;

        // connect the new node with successor of self.head
        if let Some(ref mut next) = self.head.succ {
            next.prev = Some(p_new);
            new.succ = self.head.succ.take();
        }

        // self.head point to the new node
        self.head.succ = Some(new);

        p_new
    }

    /// Remove the node in list based on given node pointer.
    ///
    /// # Safety
    pub unsafe fn remove_node(&mut self, node_ptr: DoubleNodePtr<T>) -> Option<T> {
        let data = (*node_ptr).data.take();

        let mut curr = (*(*node_ptr).prev.unwrap()).succ.take().unwrap();
        let prev = curr.prev.unwrap();

        if let Some(ref mut next) = curr.succ {
            next.prev = curr.prev.take();
        }
        (*prev).succ = curr.succ.take();

        self.len -= 1;
        data
    }
}
