use libc::{ftok, sh, shmctl, shmget, IPC_RMID};

fn remove_shared_memory() {
    let filename = "/tmp";
    unsafe {
        if let key = ftok(&filename, 1) == -1 {
            panic!("ftok failed");
        }
        if let shmid = shmget(key, 1024, 0777) == -1 {
            panic!("shmget failed");
        }
        if shmctl(shmid, IPC_RMID, 0) == -1 {
            panic!("shmctl failed");
        }
        println!("ID shared memory: {} eliminado", shmid);
    }
}

fn main() {
    remove_shared_memory();
    println!("Hello, world!");
}
