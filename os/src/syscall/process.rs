//! Process management syscalls
use crate::{
    config::*, task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER}, timer::{ get_time_ms, get_time_us}
};

///TimeVal
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// sec
    pub sec: usize,
    /// usec
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

impl TaskInfo {
    ///taskinfo init
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe{
        let _time =get_time_ms();
        let inner=TASK_MANAGER.inner.exclusive_access();
        let now_app=inner.current_task;
        let mut now_task=inner.tasks[now_app];
        now_task.task_info.time=_time-now_task.begintime;
        (*_ti)=now_task.task_info;
    }
    0
}
