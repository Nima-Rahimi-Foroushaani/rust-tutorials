use std::sync::Mutex;

pub fn main() {
    let m_i32 = Mutex::new(0);
    let g_i32 = m_i32.lock();
}
