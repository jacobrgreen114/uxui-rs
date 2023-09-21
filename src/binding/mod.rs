use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

pub struct Delegate<Ret, Args> {
    func: Arc<RefCell<dyn FnMut(Args) -> Ret>>,
}

impl<Ret, Args> PartialEq<Self> for Delegate<Ret, Args> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.func, &other.func)
    }
}

impl<Ret, Args> Eq for Delegate<Ret, Args> {}

impl<Ret, Args> Delegate<Ret, Args> {
    pub fn new<F: FnMut(Args) -> Ret + 'static>(func: F) -> Self {
        Self {
            func: Arc::new(RefCell::new(func)),
        }
    }

    pub fn invoke(&self, args: Args) -> Ret {
        (self.func.borrow_mut())(args)
    }

    fn duplicate(&self) -> Self {
        Self {
            func: self.func.clone(),
        }
    }
}

type Action<T> = Delegate<(), T>;

#[derive(Default)]
pub struct Event<Args>
where
    Args: Clone,
{
    callbacks: Vec<Action<Args>>,
}

impl<Args> Event<Args>
where
    Args: Clone,
{
    pub fn new() -> Self {
        Self {
            callbacks: Default::default(),
        }
    }

    pub fn invoke(&mut self, args: &Args) {
        for callback in self.callbacks.iter() {
            // todo : reimplement without clone
            callback.invoke(args.clone());
        }
    }
}

struct PropertyInternal<T>
where
    T: Clone,
{
    value: T,
    bindings: Event<T>,
}

impl<T> PropertyInternal<T>
where
    T: Clone,
{
    fn new(value: T) -> Self {
        Self {
            value,
            bindings: Event::new(),
        }
    }

    fn set(&mut self, value: T) {
        self.value = value;
        self.bindings.invoke(&self.value);
    }
}

pub struct Property<T>
where
    T: Clone,
{
    internal: Arc<RwLock<PropertyInternal<T>>>,
}

impl<T> Property<T>
where
    T: Clone,
{
    fn new(value: T) -> Self {
        Self {
            internal: Arc::new(RwLock::new(PropertyInternal::new(value))),
        }
    }

    pub fn set(&mut self, value: T) {
        let mut lock = self.internal.write().unwrap();
        lock.set(value);
    }

    pub fn bind(&mut self, callback: Action<T>) -> PropertyBinding<T> {
        let mut lock = self.internal.write().unwrap();
        lock.bindings.callbacks.push(callback.duplicate());
        PropertyBinding {
            internal: self.internal.clone(),
            delegate: callback,
        }
    }
}

impl<T> Default for Property<T>
where
    T: Default + Clone,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

pub struct PropertyBinding<T>
where
    T: Clone,
{
    internal: Arc<RwLock<PropertyInternal<T>>>,
    delegate: Action<T>,
}

impl<T> PropertyBinding<T> where T: Clone {}

impl<T> Drop for PropertyBinding<T>
where
    T: Clone,
{
    fn drop(&mut self) {
        let mut lock = self.internal.write().unwrap();
        lock.bindings.callbacks.retain(|c| *c != self.delegate);
    }
}

pub enum Bindable<T: Clone> {
    Unbound(T),
    Bound(PropertyBinding<T>),
}

// impl<T> Bindable<T> {
//     pub fn new(value: T) -> Self {
//         Self::Unbound(value)
//     }
//
//     pub fn bind(&mut self, binding: PropertyBinding<T>) {
//         *self = Self::Bound(binding);
//     }
// }

pub type StringProperty = Property<String>;
pub type StringPropertyBinding = PropertyBinding<String>;
pub type BindableString = Bindable<String>;
