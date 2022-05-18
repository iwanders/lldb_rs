use crate::api::ffi::lldb as bindings;
use crate::autocxx::prelude::*;
use std::pin::Pin;

/// A wrapper for the SBEvent object, to make this printable and allow for easier inspection.
pub struct Event {
    event: UniquePtr<bindings::SBEvent>,
}

impl Event {
    /// Create a new Event wrapping the provided SBEvent.
    pub fn from(event: UniquePtr<bindings::SBEvent>) -> Self {
        Event { event }
    }

    /// Create a new wrapped event, holding a default initialised SBEvent.
    pub fn new() -> Self {
        Event {
            event: bindings::SBEvent::new().within_unique_ptr(),
        }
    }

    /// Retrieve the event_type for this event.
    pub fn event_type(&self) -> bindings::StateType {
        bindings::SBProcess::GetStateFromEvent(self.event.as_ref().unwrap())
    }

    /// Return a pinned mutable reference to the internal object.
    pub fn pin_mut(&mut self) -> Pin<&mut bindings::SBEvent> {
        self.event.pin_mut()
    }
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // https://lldb.llvm.org/python_api/lldb.SBEvent.html

        // Get the event process state.
        let process_state = self.event_type();
        write!(f, "event_type: {:?} ", process_state)?;

        // Get the full description
        let mut s = bindings::SBStream::new().within_unique_ptr();
        self.event.as_ref().unwrap().GetDescription1(s.pin_mut());
        let event_str = unsafe { std::ffi::CStr::from_ptr(s.as_mut().unwrap().GetData()) };
        write!(f, "event: {:?}", event_str)?;

        // Finally, check if there is an event string to use.
        let event_str = bindings::SBEvent::GetCStringFromEvent(&self.event.as_ref().unwrap());
        if !event_str.is_null() {
            let z = unsafe { std::ffi::CStr::from_ptr(event_str) };
            write!(f, "event string: {:?}", z)?;
        }
        Ok(())
    }
}

// We could really benefit from:
// https://github.com/rust-lang/rust/pull/96709
// To support bindings that support both Pin<Box<T>> as well as UniquePtr<T>

// Create these two in case we ever want the wrappers to use another container easily.
type Carrier<T> = Pin<Box<T>>;
fn within<T: autocxx::WithinBox>(z: T) -> Wrapped<T::Inner> {
    wrapped(z.within_box())
}
pub fn wrapped<T >(item: Carrier<T>) -> Wrapped<T> {
    Wrapped{item}
}

// We need a wrapper type we own, such that we can implement external traits such as std::fmt::Debug
// To implement external traits, we can implement them for on this wrapped type we own;
// but we can still use methods of our PinMut implementing trait, which means not everything
// needs to be wrapped all the time.
// We will implement our traits with convenience methods extending something that implements
// autocxx::PinMut<T>. This means that we can use our traits on UniquePtr<T>,
// Pin<Box<T>> as well as on our own wrapper which also implements autocxx::PinMut<T>
// Say SBDebugger has a method that provides SBTarget, but we didn't wrap that, we can still do:
// let foo = wrapped(SBDebugger::Create());
// let t = foo.GetTarget().within_box().my_target_method()
// where my_target_method is something that's implemented by our wrapper for Target, that uses the
// PinMut implementation. This means that even if our SBDebugger doesn't provide full coverage of
// the methods in SBDebugger this is not a problem to use the helper methods for Target.
// Returning wrapped versions is still better, as they would allow us to use the traits external to
// this crate, like std::fmt::Debug, but going with this for now as this provides a good way forward
// when the API isn't fully covered (which, it likely will never be).
// And this entire system also avoids having to duplicate or use a macro to implement the
// convenience methods for all three wrapping types.

pub struct Wrapped<T> {
    item: Carrier<T>
}
impl<T> Wrapped<T> 
{
    pub fn new(item: Carrier<T>) -> Self
    {
        Wrapped::<T>{item}
    }
}

/// For this type, implement AsRef, prerequisite for autocxx::PinMut
impl<T> std::convert::AsRef<T> for Wrapped<T> 
{
    fn as_ref(&self) -> &T
    {
        self.item.as_ref().get_ref()
    }
}
/// Implement PinMut for our wrapper, this makes all methods accessible.
impl<T> autocxx::PinMut<T> for Wrapped<T> 
{
    fn pin_mut(&mut self) -> Pin<&mut T> {
        self.item.as_mut()
    }
}

/// Macro to implement pin_mut() for our types in UniquePtr<T> and Pin<Box<T>>
macro_rules! handle_box_and_uniqueptr {
    ($t:ty) => {
        impl std::convert::AsRef<$t> for UniquePtr<$t> {
            fn as_ref(&self) -> &$t {
                self.as_ref().expect("was nullptr")
            }
        }

        impl autocxx::PinMut<$t> for UniquePtr<$t> {
            fn pin_mut(&mut self) -> Pin<&mut $t> {
                self.pin_mut()
            }
        }

        impl std::convert::AsRef<$t> for Pin<Box<$t>> {
            fn as_ref(&self) -> &$t {
                self.as_ref().get_ref()
            }
        }

        impl autocxx::PinMut<$t> for Pin<Box<$t>> {
            fn pin_mut(&mut self) -> Pin<&mut $t> {
                self.as_mut()
            }
        }
    };
}

// Actual implementations now follow.
// Todo; check const correctness.

handle_box_and_uniqueptr!(bindings::SBProcess);
trait Process: autocxx::PinMut<bindings::SBProcess> {
    fn thread(&mut self, id: usize) -> Wrapped<bindings::SBThread> {
        within(self.pin_mut().GetThreadAtIndex(id))
    }
}
impl<T> Process for T where T: autocxx::PinMut<bindings::SBProcess> {}

handle_box_and_uniqueptr!(bindings::SBThread);
trait Thread: autocxx::PinMut<bindings::SBThread> {
    fn frame(&mut self, id: u32) -> Wrapped<bindings::SBFrame> {
        within(self.pin_mut().GetFrameAtIndex(id))
    }
}
impl<T> Thread for T where T: autocxx::PinMut<bindings::SBThread> {}

handle_box_and_uniqueptr!(bindings::SBFrame);
trait Frame: autocxx::PinMut<bindings::SBFrame> {
    fn find_register(&mut self, name: &str) -> Wrapped<bindings::SBValue> {
        let reg = std::ffi::CString::new(name).expect("no null bytes expected");
        within(unsafe { self.pin_mut().FindRegister(reg.as_ptr()) })
    }
}
impl<T> Frame for T where T: autocxx::PinMut<bindings::SBFrame> {}

handle_box_and_uniqueptr!(bindings::SBValue);
trait Value: autocxx::PinMut<bindings::SBValue> {
    fn get_value_as_unsigned(&mut self) -> Result<u64, Box<Wrapped<bindings::SBError>>> {
        let mut e = within(bindings::SBError::new());
        let res = self.pin_mut().GetValueAsUnsigned(e.pin_mut(), 0);
        if e.is_success() {
            return Ok(res);
        }
        Err(Box::new(e))
    }
}
impl<T> Value for T where T: autocxx::PinMut<bindings::SBValue> {}


handle_box_and_uniqueptr!(bindings::SBError);
trait Error: autocxx::PinMut<bindings::SBError> {

    /// Returns internal objects Fail() method.
    fn is_fail(&self) -> bool {
        self.as_ref().Fail()
    }

    /// Returns internal objects Success() method.
    fn is_success(&self) -> bool {
        self.as_ref().Success()
    }
    /// Return a string representation for this error.
    fn get_str(&self) -> &str {
        let err_str = bindings::SBError::GetCString(&self.as_ref());
        if err_str.is_null() {
            return "no error string";
        }
        let z = unsafe { std::ffi::CStr::from_ptr(err_str) };
        z.to_str().expect("should be ascii")
    }

    /// Return the error type.
    fn get_type(&self) -> bindings::ErrorType {
        self.as_ref().GetType()
    }
}
impl<T> Error for T where T: autocxx::PinMut<bindings::SBError> {}

impl std::fmt::Debug for Wrapped<bindings::SBError>  {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.get_str())
    }
}

impl std::error::Error for Wrapped<bindings::SBError>{
    fn description(&self) -> &str {
        self.get_str()
    }
}
impl std::fmt::Display for Wrapped<bindings::SBError>{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error ({:?}): {}", self.get_type(), self.get_str())
    }
}




#[cfg(test)]
mod test {
    use super::*;
    use crate::api::ffi::lldb;
    use autocxx::prelude::*;

    // And then, the wrappers allow us to write things nice and concise:
    #[test]
    fn test_process_box() {
        // This operates without our wrapper on the first call.
        let mut p = lldb::SBProcess::new().within_box();
        let mut t = p.thread(0);
        let mut f = t.frame(0);
        let mut reg = f.find_register("edx");
        let v = reg.get_value_as_unsigned();
        assert!(v.is_err());
    }

    #[test]
    fn test_process_wrapped_box() {
        // This operates on our wrapper in the first call.
        let mut p = wrapped(lldb::SBProcess::new().within_box());
        let mut t = p.thread(0);
        let mut f = t.frame(0);
        let mut reg = f.find_register("edx");
        let v = reg.get_value_as_unsigned();
        assert!(v.is_err());
    }
    #[test]
    fn test_process_unique_ptr() {
        let mut p = lldb::SBProcess::new().within_unique_ptr();
        let mut t = p.thread(0);
        let mut f = t.frame(0);
        let mut reg = f.find_register("edx");
        let v = reg.get_value_as_unsigned();
        assert!(v.is_err());
    }

    #[test]
    fn test_error()
    {
        let e = wrapped(lldb::SBError::new().within_box());
        println!("{}", e);
        println!("{:?}", e);
        let z: Box<dyn std::error::Error> = Box::new(e);
        println!("{}", z);
        println!("{:?}", z);
    }
}
