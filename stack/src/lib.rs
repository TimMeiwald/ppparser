mod stack_trait;
mod basic_stack;
mod noop_stack;
mod printer_stack;
pub use stack_trait::Stack;
pub use noop_stack::NoopStack;
pub use printer_stack::PrinterStack;