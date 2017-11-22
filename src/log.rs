//! Traits and types for logging errors during compilation.

use std::fmt::Display;
use std::io::Write;

/// Log Priority level.
#[derive(Debug, Clone, PartialEq)]
pub enum LogPriority {
    /// Non-error log output
    Message,
    /// Warning-level log output
    Warn,
    /// Error-level log output
    Error
}
/// Log listener object for accepting log messages of type `T`, buffering them, and outputting them
/// to output stream `O` and error stream `E` as needed.
pub struct LogListener<T: Display, O, E> {
    messages: Vec<(T, LogPriority)>,
    cout: O,
    cerr: E,
}
impl<T: Display, O: Write, E: Write> LogListener<T, O, E> {
    /// Create a new log listener with empty buffer.
    pub fn new(cout: O, cerr: E) -> LogListener<T, O, E> {
        LogListener {
            messages: Vec::new(),
            cout: cout,
            cerr: cerr,
        }
    }

    /// Adds a log message with `LogPriority::Message` to the message buffer.
    pub fn log(&mut self, msg: T) {
        println!("log: {}", msg);
        self.messages.push((msg, LogPriority::Message));
    }

    /// Adds a log message with `LogPriority::Warn` to the message buffer.
    pub fn warn(&mut self, msg: T) {
        println!("warn: {}", msg);
        self.messages.push((msg, LogPriority::Warn));
    }

    /// Adds a log message with `LogPriority::Error` to the message buffer.
    pub fn error(&mut self, msg: T) {
        println!("error: {}", msg);
        self.messages.push((msg, LogPriority::Error));
    }

    /// Flushes the current message buffer, printing out the current contents to the output and
    /// error streams. Returns the highest priority log message that was outputted.
    pub fn flush(&mut self) -> Option<LogPriority> {
        let mut highest_prio = None;
        for (msg, prio) in self.messages.drain(..) {
            match prio {
                LogPriority::Message => {
                    writeln!(self.cout, "{}", msg).unwrap();
                },
                LogPriority::Warn | LogPriority::Error => {
                    writeln!(self.cerr, "{}", msg).unwrap();
                }
            }
            highest_prio = Some(prio);
        }
        highest_prio
    }
}


