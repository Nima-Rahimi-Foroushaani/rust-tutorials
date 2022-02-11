use std::{io, sync};
#[derive(Clone)]
pub struct DiagnosticSink(sync::Arc<sync::Mutex<Vec<u8>>>);

impl DiagnosticSink {
    pub fn new(arc: sync::Arc<sync::Mutex<Vec<u8>>>) -> Self {
        DiagnosticSink(arc)
    }
}
impl io::Write for DiagnosticSink {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.lock().unwrap().flush()
    }
}
