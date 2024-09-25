extern crate libc;
use libc::{ftok, shmat, shmctl, shmdt, shmget, IPC_RMID};

use std::ffi::CString;
use std::ptr;

const FILE_KEY: &str = "/bin/cat";
const SHM_KEY: i32 = 9;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Registro {
    pub id: i32,
    pub producto: [u8; 50],
    pub marca: [u8; 50],
    pub precio: i32,
    pub cantidad: i32,
}

fn borrar_shared_memory() {
    let file_key = CString::new(FILE_KEY).expect("CString::new failed");

    let key = unsafe { ftok(file_key.as_ptr(), SHM_KEY) };
    if key == -1 {
        panic!("Hubo un error al generar la clave con ftok");
    }
    let smh_id = unsafe { shmget(key, 0, 0) };
    if smh_id == -1 {
        panic!("Hubo un error al obtener la memoria compartida");
    }
    let shm_ptr = unsafe { shmat(smh_id, ptr::null(), 0) } as *mut Registro;
    if shm_ptr.is_null() {
        panic!("No se pudo acceder a la memoria compartida");
    }
    unsafe {
        shmdt(shm_ptr as *const _);
        shmctl(smh_id, IPC_RMID, ptr::null_mut());
    }
}

fn main() {
    borrar_shared_memory();
}
