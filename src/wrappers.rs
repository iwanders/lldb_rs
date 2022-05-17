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

/// A wrapper for the SBError object, to make this printable and a true error.
pub struct Error {
    err: UniquePtr<bindings::SBError>,
}

impl Error {
    /// Create a new Event wrapping the provided SBError.
    pub fn from(err: UniquePtr<bindings::SBError>) -> Self {
        Error { err }
    }

    /// Create a new wrapped error, holding a default initialised SBError.
    pub fn new() -> Self {
        Error {
            err: bindings::SBError::new().within_unique_ptr(),
        }
    }

    /// Convert self into a boxed version of Self.
    pub fn into_box(self) -> Box<Error> {
        Box::new(self)
    }

    /// Return a pinned mutable reference to the internal object.
    pub fn pin_mut(&mut self) -> Pin<&mut bindings::SBError> {
        self.err.pin_mut()
    }

    /// Returns internal objects Fail() method.
    pub fn is_fail(&self) -> bool {
        if self.is_null() {
            panic!("Error may never be null");
        }
        self.err.as_ref().unwrap().Fail()
    }

    /// Returns internal objects Success() method.
    pub fn is_success(&self) -> bool {
        if self.is_null() {
            panic!("Error may never be null");
        }
        self.err.as_ref().unwrap().Success()
    }

    /// Check if the internal object is_null.
    fn is_null(&self) -> bool {
        self.err.is_null()
    }

    /// Return a string representation for this error.
    pub fn get_str(&self) -> &str {
        let err_str = bindings::SBError::GetCString(&self.err.as_ref().unwrap());
        if err_str.is_null() {
            return "no error string";
        }
        let z = unsafe { std::ffi::CStr::from_ptr(err_str) };
        return z.to_str().expect("should be ascii");
    }

    /// Return the error type.
    pub fn get_type(&self) -> bindings::ErrorType {
        self.err.as_ref().unwrap().GetType()
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        self.get_str()
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error ({:?}): {}", self.get_type(), self.get_str())
    }
}
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.get_str())
    }
}


// We could really benefit from:
// https://github.com/rust-lang/rust/pull/96709
// To support bindings that support both Pin<Box<T>> as well as UniquePtr<T>

// Create these two in case we ever want the wrappers to use another container easily.
type Carrier<T> = UniquePtr<T>;
fn wrap<T: autocxx::WithinUniquePtr> (z: T) -> Carrier<T::Inner>
{
    z.within_unique_ptr()
}

macro_rules! handle_box_and_uniqueptr {
    ($t:ty) => {
        impl std::convert::AsRef<$t>  for UniquePtr<$t>
        {   fn as_ref(&self) -> &$t
            {
                self.as_ref().expect("was nullptr")
            }
        }

        impl autocxx::PinMut<$t>  for UniquePtr<$t>
        {
            fn pin_mut(&mut self) -> Pin<&mut $t> {
                self.pin_mut()
            }

        }

        impl std::convert::AsRef<$t>  for Pin<Box<$t>>
        {   fn as_ref(&self) -> &$t
            {
                self.as_ref().get_ref()
            }
        }

        impl autocxx::PinMut<$t>  for Pin<Box<$t>>
        {
            fn pin_mut(&mut self) -> Pin<&mut $t> {
                self.as_mut()
            }
        }
    }
}

handle_box_and_uniqueptr!(bindings::SBProcess);
// Actual implementations now follow.
trait Process : autocxx::PinMut<bindings::SBProcess> {
    fn thread(&mut self, id: usize) -> Carrier<bindings::SBThread>
    {
        wrap(self.pin_mut().GetThreadAtIndex(id))
    }
}
impl<T> Process for T where T: autocxx::PinMut<bindings::SBProcess> { }


handle_box_and_uniqueptr!(bindings::SBThread);
trait Thread : autocxx::PinMut<bindings::SBThread> {
    fn frame(&mut self, id: u32) -> Carrier<bindings::SBFrame>
    {
        wrap(self.pin_mut().GetFrameAtIndex(id))
    }
}
impl<T> Thread for T where T: autocxx::PinMut<bindings::SBThread> { }


handle_box_and_uniqueptr!(bindings::SBFrame);
trait Frame : autocxx::PinMut<bindings::SBFrame> {
    fn find_register(&mut self, name: &str) -> Carrier<bindings::SBValue>
    {
        let reg = std::ffi::CString::new(name).expect("no null bytes expected");
        wrap(unsafe{self.pin_mut().FindRegister(reg.as_ptr())})
    }
}
impl<T> Frame for T where T: autocxx::PinMut<bindings::SBFrame> { }


handle_box_and_uniqueptr!(bindings::SBValue);
trait Value : autocxx::PinMut<bindings::SBValue> {
    fn get_value_as_unsigned(&mut self) -> Result<u64, Box<Error>>
    {
        let mut e =  Error::new();
        let res = self.pin_mut().GetValueAsUnsigned(e.pin_mut(), 0 );
        if e.is_success()
        {
            return Ok(res);
        }
        Err(e.into_box())
    }
}
impl<T> Value for T where T: autocxx::PinMut<bindings::SBValue> { }



#[cfg(test)]
mod test {
    use super::*;
    use crate::api::ffi::lldb;
    use autocxx::prelude::*;

    #[test]
    fn test_process_box() {
        let mut p = lldb::SBProcess::new().within_box();
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

}

