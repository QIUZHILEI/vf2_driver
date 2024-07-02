use core::time::Duration;

pub const MTIME_BASE: usize = 0x0200_BFF8;
pub const TIME_BASE: usize = 4000000;

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    deadline: usize,
}

impl Timer {
    pub fn start(deadline: Duration) -> Self {
        Self {
            deadline: read_tick() + to_tick(deadline),
        }
    }

    pub fn timeout(&self) -> bool {
        if read_tick() >= self.deadline {
            true
        } else {
            false
        }
    }
}

unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}

pub fn delay(dur: Duration) {
    let limit = read_tick() + to_tick(dur);
    while read_tick() < limit {}
}

#[inline]
fn to_tick(dur: Duration) -> usize {
    (dur.as_micros() as usize) * TIME_BASE / 1000000
}

#[inline]
fn read_tick() -> usize {
    unsafe { (MTIME_BASE as *const usize).read_volatile() }
}
