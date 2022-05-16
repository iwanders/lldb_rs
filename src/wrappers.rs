use crate::api::ffi::lldb as bindings;
use crate::autocxx::prelude::*;
use std::pin::Pin;

/// A wrapper for the SBEvent object, to make this printable and allow for easier inspection.
pub struct Event {
    event: UniquePtr<bindings::SBEvent>,
}

impl Event {
    /// Create a new Event wrapping the provided SBEvent.
    pub fn new(event: UniquePtr<bindings::SBEvent>) -> Self {
        Event { event }
    }

    /// Create a new wrapped event, holding a default initialised SBEvent.
    pub fn create() -> Self {
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
    pub fn new(err: UniquePtr<bindings::SBError>) -> Self {
        Error { err }
    }

    /// Create a new wrapped error, holding a default initialised SBError.
    pub fn create() -> Self {
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
