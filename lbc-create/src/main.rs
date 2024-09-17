use libc::{ftok, shmat, shmget, IPC_CREAT};

fn set_shared_memory() {
    let filename = "/tmp";
    unsafe {
        if let key = ftok(&filename, 1) == -1 {
            panic!("ftok failed");
        }
        if let shmid = shmget(key, size_of::<i32>(), IPC_CREAT | 0o777) == -1 {
            panic!("shmget failed");
        }
        println!("ID shared memory: {}", shmid);
        if let ptr_counter = shmat(shmid, ptr::null(), 0) == -1 {
            panic!("shmat failed");
        }
        *(ptr as *mut i32 = 0);
        println!("counter: {}", ptr_counter);
    }
}

fn main() {
    println!("Hello, world!");
}
