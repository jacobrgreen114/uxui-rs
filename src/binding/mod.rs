use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

struct PropertyInternal<T> {
    value: T,
}

pub struct Property<T> {
    internal: Rc<RefCell<PropertyInternal<T>>>,
}

impl<T> Property<T> {
    fn new(value: T) -> Self {
        Self {
            internal: Rc::new(RefCell::new(PropertyInternal { value })),
        }
    }
}

impl<T> Default for Property<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub struct PropertyBinding<T> {
    internal: Arc<RwLock<PropertyInternal<T>>>,
}

pub enum Bindable<T> {
    Unbound(T),
    Bound(PropertyBinding<T>),
}

impl<T> Bindable<T> {
    pub fn new(value: T) -> Self {
        Self::Unbound(value)
    }

    pub fn bind(&mut self, binding: PropertyBinding<T>) {
        *self = Self::Bound(binding);
    }
}

pub type StringProperty = Property<String>;
pub type StringPropertyBinding = PropertyBinding<String>;
pub type BindableString = Bindable<String>;
