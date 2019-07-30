//! # trapper
//! 
//! Trapper (or transparent wrapper) allows for the creation of transparent type wrappers, 
//! that is types which are transparent and can be wrapped and unwrapped for zero cost.


/// A type wrapper. This trait provides methods for converting between a wrapper and its 
/// inner type. It should only be implemented by types through the [`newtype`](macro.newtype.html) macro. If it must
/// be implemented manually, the type should have transparent representation to be safe.
pub unsafe trait Wrapper: Sized {
    /// The inner wrapped type
    type Inner: Sized;

    /// Wraps the value, returning a new instance of the wrapper.
    /// 
    /// # Example
    /// 
    /// ```
    /// use trapper::{Wrapper, newtype};
    /// newtype!(#[derive(PartialEq, Debug)] type NumberWrapper(i32));
    /// 
    /// let wrapper = NumberWrapper::wrap(12);
    /// let other = NumberWrapper::wrap(12);
    /// assert_eq!(wrapper, other);
    /// ```
    fn wrap(inner: Self::Inner) -> Self;
    /// Unwraps the wrapper, returning its inner value.
    /// 
    /// # Example
    /// 
    /// ```
    /// use trapper::{Wrapper, newtype};
    /// newtype!(type NumberWrapper(i32));
    /// 
    /// let wrapper = NumberWrapper::wrap(12);
    /// assert_eq!(wrapper.unwrap(), 12);
    /// ```
    fn unwrap(self) -> Self::Inner;

    /// Wraps a shared reference to the value in the wrapper type
    /// 
    /// # Example
    /// 
    /// ```
    /// use trapper::{Wrapper, newtype};
    /// newtype!(type NumberWrapper(i32));
    /// 
    /// let number = 12;
    /// let wrapper: &NumberWrapper = NumberWrapper::wrap_ref(&number);
    /// ```
    fn wrap_ref(inner: &Self::Inner) -> &Self {
        unsafe { &*(inner as *const Self::Inner as *const Self) }
    }
    /// Wraps a unique reference to the value in the wrapper type
    /// 
    /// # Example
    /// 
    /// ```
    /// use trapper::{Wrapper, newtype};
    /// newtype!(type NumberWrapper(i32));
    /// 
    /// let mut number = 12;
    /// let wrapper: &mut NumberWrapper = NumberWrapper::wrap_mut(&mut number);
    /// *wrapper = NumberWrapper::wrap(13);
    /// 
    /// assert_eq!(number, 13);
    /// ```
    fn wrap_mut(inner: &mut Self::Inner) -> &mut Self {
        unsafe { &mut *(inner as *mut Self::Inner as *mut Self) }
    }

    /// Unwraps a shared reference to the wrapper, exposing the underlying type
    /// 
    /// # Example
    /// 
    /// ```
    /// use trapper::{Wrapper, newtype};
    /// newtype!(type NumberWrapper(i32));
    /// 
    /// let wrapper = NumberWrapper::wrap(12);
    /// 
    /// assert_eq!(*wrapper.unwrap_ref(), 12);
    /// ```
    fn unwrap_ref(&self) -> &Self::Inner {
        unsafe { &*(self as *const Self as *const Self::Inner) }
    }
    /// Unwraps a unique reference to the wrapper, exposing the underlying type
    /// 
    /// # Example
    /// 
    /// ```
    /// use trapper::{Wrapper, newtype};
    /// newtype!(#[derive(PartialEq, Debug)] type NumberWrapper(i32));
    /// 
    /// let mut wrapper = NumberWrapper::wrap(12);
    /// *wrapper.unwrap_mut() = 13;
    /// 
    /// assert_eq!(wrapper, NumberWrapper::wrap(13));
    /// ```
    fn unwrap_mut(&mut self) -> &mut Self::Inner {
        unsafe { &mut *(self as *mut Self as *mut Self::Inner) }
    }
}

/// Creates a new wrapper type. This type is transparent and implements [`Wrapper`](trait.Wrapper.html)
/// 
/// # Examples
/// 
/// ```
/// use trapper::newtype;
/// 
/// newtype!(type BasicNumber(i32));
/// newtype!(pub type WithVisibility(i32));
/// newtype!(pub type WithLifetimes<'a>(std::io::StderrLock<'a>));
/// ```
#[macro_export]
macro_rules! newtype { 
    ($(#[$a:meta])* $visbility:vis type $name:ident$(<$($l:lifetime),+>)?($inner:ty)) => {
        $(#[$a]
        )*
        #[repr(transparent)]
        $visbility struct $name$(<$($l),+>)?($inner);
        unsafe impl$(<$($l),+>)? $crate::Wrapper for $name$(<$($l),+>)? {
            type Inner = $inner;

            fn wrap(inner: Self::Inner) -> Self { Self(inner) }
            fn unwrap(self) -> Self::Inner { self.0 }
        }
    };
}

#[cfg(test)]
mod tests {
    newtype!(#[allow(dead_code)] type InMod(i32));

    #[test]
    fn in_function() {
        newtype!(#[allow(dead_code)] type InFunction(i32));
    }
}