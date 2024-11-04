use alloc::vec::Vec;

/// SyncResource
pub struct SyncResource {
    // [i] = res available, i = res id
    available: Vec<usize>,
    // [i, j] = res allocated, i = tid, j = res id
    allocation: Vec<Vec<usize>>,
    // [i, j] = res needed, i = tid, j = res id
    need: Vec<Vec<usize>>,
    thread_num: usize,
    resource_num: usize,
}

impl SyncResource {
    /// new
    pub fn new() -> Self {
        Self {
            available: Vec::new(),
            allocation: Vec::new(),
            need: Vec::new(),
            thread_num: 0,
            resource_num: 0,
        }
    }

    fn ensure_threads(&mut self, tid: usize) {
        if tid < self.thread_num {
            return;
        }
        for _ in self.thread_num..tid + 1 {
            let mut resource = Vec::with_capacity(self.resource_num);
            resource.resize(self.resource_num, 0);
            self.allocation.push(resource.clone());
            self.need.push(resource);
        }
        self.thread_num = tid + 1;
    }

    /// add new resource
    pub fn add_new_resource(&mut self, rid: usize, amount: usize) {
        if rid < self.resource_num {
            self.available[rid] += amount;
            return;
        }
        self.resource_num = rid + 1;
        self.available.resize(self.resource_num, 0);
        self.available[rid] = amount;

        for allocated in self.allocation.iter_mut() {
            allocated.resize(self.resource_num, 0);
        }
        for need in self.need.iter_mut() {
            need.resize(self.resource_num, 0);
        }
    }

    /// remove resource
    pub fn remove_resource(&mut self, rid: usize) {
        assert!(rid < self.resource_num);
        self.available[rid] = 0;
    }

    /// need
    pub fn add_need(&mut self, tid: usize, rid: usize, amount: usize) {
        self.ensure_threads(tid);
        self.need[tid][rid] += amount;
    }

    /// need
    pub fn remove_need(&mut self, tid: usize, rid: usize, amount: usize) {
        self.ensure_threads(tid);
        self.need[tid][rid] -= amount;
    }

    /// alloc
    pub fn alloc(&mut self, tid: usize, rid: usize, amount: usize) {
        self.ensure_threads(tid);
        self.allocation[tid][rid] += amount;
        self.available[rid] -= amount;
    }

    /// dealloc
    pub fn dealloc(&mut self, tid: usize, rid: usize, amount: usize) {
        self.ensure_threads(tid);
        self.allocation[tid][rid] -= amount;
        self.available[rid] += amount;
    }

    /// check deadlock
    pub fn detect_deadlock(&self) -> bool {
        let mut work = self.available.clone();
        let mut finish = Vec::with_capacity(self.thread_num);
        finish.resize(self.thread_num, false);
        loop {
            let mut changed = false;
            let mut can_finish = true;
            for tid in 0..self.thread_num {
                if finish[tid] {
                    continue;
                }
                let done = (0..self.resource_num).all(|rid| self.need[tid][rid] <= work[rid]);
                if done {
                    for rid in 0..self.resource_num {
                        work[rid] += self.allocation[tid][rid];
                    }
                    finish[tid] = true;
                    changed = true;
                } else {
                    can_finish = false;
                }
            }
            if can_finish {
                return false;
            }
            if !changed {
                return true;
            }
        }
    }
}
