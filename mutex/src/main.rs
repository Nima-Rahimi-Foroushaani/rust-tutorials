#![allow(dead_code)]

mod sys {
    pub mod locks {
        use std::process::abort;

        pub struct Mutex();

        impl Mutex {
            pub fn new() -> Mutex
//@ req thread_token(?t);
        //@ ens thread_token(t) &*& Mutex_own(t);
            {
                abort();
            }

            pub unsafe fn lock<'a>(&'a self)
            //@ req thread_token(?t) &*& lifetime_token(?a) &*& Mutex_share(a, t, self);
            //@ ens thread_token(t) &*& lifetime_token(a);
            {
                abort();
            }

            pub unsafe fn unlock<'a>(&'a self) {
                abort();
            }

            pub unsafe fn try_lock<'a>(&'a self) -> bool {
                abort();
            }
        }
    }
}

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

struct MutexU32 {
    inner: sys::locks::Mutex,
    data: UnsafeCell<u32>,
}

struct MutexGuardU32<'a> {
    lock: &'a MutexU32,
}

impl MutexU32 {
    pub fn new(v: u32) -> MutexU32 {
        MutexU32 {
            inner: sys::locks::Mutex::new(),
            data: UnsafeCell::new(v),
        }
    }

    pub fn lock<'a>(&'a self) -> MutexGuardU32<'a> {
        unsafe {
            self.inner.lock();
            MutexGuardU32::new(self)
        }
    }

    pub fn unlock<'a>(guard: MutexGuardU32<'a>) {
        drop(guard);
    }

    pub fn into_inner(self) -> u32 {
        let data = self.data.into_inner();
        data
    }
}

impl<'mutex> MutexGuardU32<'mutex> {
    unsafe fn new(lock: &'mutex MutexU32) -> MutexGuardU32<'mutex> {
        MutexGuardU32 { lock }
    }
}

impl<'a> Deref for MutexGuardU32<'a> {
    type Target = u32;
    fn deref(&self) -> &u32 {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a> DerefMut for MutexGuardU32<'a> {
    fn deref_mut(&mut self) -> &mut u32 {
        unsafe { &mut *self.lock.data.get() }
    }
}

fn main() {
    println!("Hello, world!");
}
