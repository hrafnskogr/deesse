use std::cell::UnsafeCell;
use std::cell::Cell;

pub struct OnceCell<T>
{
    inner: UnsafeCell<Option<T>>,
    init: Cell<fn() -> T>,
}

impl<T> OnceCell<T>
{
    pub const fn new(f: fn() -> T) -> OnceCell<T>
    {
        OnceCell { inner: UnsafeCell::new(None), init: Cell::new(f) }
    }

    pub fn get(&self) -> &T
    {
        if let Some(val) = self.get_unchecked()
        {
            return val;
        }
        let val = (self.init.get())();
        let _ = self.set(val);

        self.get_unchecked().unwrap()
    }

    pub fn get_unchecked(&self) -> Option<&T>
    {
        unsafe { &*self.inner.get() }.as_ref()
    }

    pub fn get_mut(&mut self) -> Option<&mut T>
    {
        self.inner.get_mut().as_mut()
    }

    pub fn set(&self, value: T) -> Result<(), T>
    {
        let current_val = unsafe { &*self.inner.get() };
        if current_val.is_some()
        {
            return Err(value);
        }

        let current_val = unsafe { &mut *self.inner.get() };
        *current_val = Some(value);
        
        Ok(())
    }

    pub fn get_or_init<F>(&self, f: F) -> &T where F: FnOnce() -> T,
    {
        if let Some(val) = self.get_unchecked()
        {
            return val;
        }

        #[cold]
        fn init<F, T>(f: F) -> T where F: FnOnce() -> T
        {
            f()
        }
        let val = init(f);
        let _ = self.set(val);

        self.get_unchecked().unwrap()
    }
}

unsafe impl<T> Sync for OnceCell<T> {}
