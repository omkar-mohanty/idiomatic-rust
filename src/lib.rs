use std::{cell::RefCell, rc::Rc};

pub type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

pub struct ListItem<T> {
    data: ItemData<T>,
    next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(data)),
            next: None,
        }
    }
}

pub struct Iter<T> {
    item: Option<ListItemPtr<T>>,
}

impl<T> Iterator for Iter<T> {
    type Item = ItemData<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.item.clone() {
            Some(ptr) => {
                self.item.clone_from(&ptr.as_ref().borrow().next);
                Some(ptr.borrow().data.clone())
            }
            None => None,
        }
    }
}

pub struct IntoIter<T> {
    item: Option<ListItemPtr<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = ItemData<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.item.clone() {
            Some(ptr) => {
                self.item.clone_from(&ptr.as_ref().borrow().next);
                Some(ptr.borrow().data.clone())
            }
            None => None,
        }
    }
}

pub struct IterMut<T> {
    item: Option<ListItemPtr<T>>,
}

impl<T> Iterator for IterMut<T> {
    type Item = ItemData<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.item.clone() {
            Some(ptr) => {
                self.item.clone_from(&ptr.as_ref().borrow().next);
                Some(ptr.borrow().data.clone())
            }
            None => None,
        }
    }
}

pub struct LinkedList<T> {
    head: ListItemPtr<T>,
}

impl<T> LinkedList<T> {
    pub fn new(data: T) -> Self {
        Self {
            head: Rc::new(RefCell::new(ListItem::new(data))),
        }
    }

    pub fn append(&mut self, data: T) {
        let mut next = self.head.clone();

        while next.as_ref().borrow().next.is_some() {
            let n = next.as_ref().borrow().next.as_ref().unwrap().clone();

            next = n;
        }

        next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(data))));
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            item: Some(self.head.clone()),
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            item: Some(self.head),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            item: Some(self.head.clone()),
        }
    }
}
