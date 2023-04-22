struct CellU32 { v: u32 }

impl CellU32 {
  pub fn new(u: u32) -> CellU32 { CellU32 { v: u } }

  pub fn get(&self) -> u32 { self.v }

  fn set(&self, u: u32) {
    let p = &self.v as *const u32 as *mut u32;
    unsafe { *p = u; }
  }
}

impl !Sync for CellU32 {}