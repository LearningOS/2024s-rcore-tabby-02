//! Process management syscalls
// use riscv::addr::BitField;

use core::mem::size_of;

use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE}, mm::{translated_byte_buffer, MapPermission, VirtAddr}, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, TASK_MANAGER
    }, timer::{  get_time_ms, get_time_us}
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
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
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let mut buffer = translated_byte_buffer(current_user_token(), _ts as *const u8, size_of::<TimeVal>());
    // Write the seconds and microseconds to the buffer.
    let time_val_ptr = buffer[0].as_mut_ptr() as *mut TimeVal;
    unsafe {
        (*time_val_ptr).sec = us / 1_000_000;
        (*time_val_ptr).usec = us % 1_000_000;
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
        let _time =get_time_ms();
        let mut inner=TASK_MANAGER.inner.exclusive_access();
        let now_app=inner.current_task;
        inner.tasks[now_app].task_info.time=_time-inner.tasks[now_app].begintime;
        //下面的translate_byte_buffer()第一个参数一定不能和前面的sys_get_time()中的一样调current_user_token(),
        //因为这里已经获取了inner，而current_user_token会再获取	inner，导致错误
        let mut buffer = translated_byte_buffer(inner.tasks[now_app].get_user_token(), _ti as *const u8, size_of::<TaskInfo>());
        let taskinfo_val_ptr = buffer[0].as_mut_ptr() as *mut TaskInfo;
        unsafe {
        (*taskinfo_val_ptr).status=inner.tasks[now_app].task_info.status;
        (*taskinfo_val_ptr).syscall_times=inner.tasks[now_app].task_info.syscall_times;
        (*taskinfo_val_ptr).time=inner.tasks[now_app].task_info.time;
        drop(inner);
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _port & !0x7 != 0||_port & 0x7 == 0||_start%PAGE_SIZE!=0{
        return -1
    }
    let mut inner=TASK_MANAGER.inner.exclusive_access();
    let now_app=inner.current_task;
    // let p=MapPermission::from(0|_port.get_bit(2) as u8|_port.get_bit(1) as u8|_port.get_bit(0) as u8);
    let mut p = MapPermission::U;
    p.set(MapPermission::R, _port  as u8 & 0b0001 == 1);
    p.set(MapPermission::W, _port >> 1 as u8 & 0b0001 == 1);
    p.set(MapPermission::X, _port >> 2 as u8 & 0b0001 == 1);
    let sva:VirtAddr=_start.into();
    if sva.aligned()!=true{
        return -1;
    }
    let e:VirtAddr=(_len+_start).into();
    let eva:VirtAddr=e.ceil().into();
    inner.tasks[now_app].memory_set.insert_framed_area(sva, eva, p)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let mut inner=TASK_MANAGER.inner.exclusive_access();
    let now_app=inner.current_task;
    let sva:VirtAddr=_start.into();
    if sva.aligned() == false {
        return -1;
    }
    let e:VirtAddr=(_len+_start).into();
    let eva:VirtAddr=e.ceil().into();
    inner.tasks[now_app].memory_set.remove_framed_area(sva, eva)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
