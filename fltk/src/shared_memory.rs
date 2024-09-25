extern crate libc;
use libc::{ftok, semop, shmat, shmget};

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

impl Registro {
    fn new(id: i32, producto: [u8; 50], marca: [u8; 50], precio: i32, cantidad: i32) -> Self {
        let registro = Registro {
            id,
            producto,
            marca,
            precio,
            cantidad,
        };
        registro
    }
}

pub fn byte_array_to_string(array: &[u8; 50]) -> String {
    let mut s = String::new();
    for &byte in array.iter() {
        if byte == 0 {
            break;
        }
        s.push(byte as char);
    }
    s
}

pub unsafe fn sm_bloq_articulo(articulo: *mut Registro, seleccion: i32) {
    let registro = unsafe { *articulo.add(seleccion as usize) };
    let sem_id = get_sem(registro.id);

    // Bloquear acceso al registro
    bloq_sem(sem_id);

    println!(
        "Semáforo bloqueado. Iniciando compra para el producto: {:?}",
        byte_array_to_string(&registro.producto)
    );
}

pub unsafe fn sm_desb_articulo(articulo: *mut Registro, seleccion: i32) {
    let registro = *articulo.add(seleccion as usize);
    let sem_id = get_sem(registro.id);

    // Desbloquear acceso al registro
    desbloquear_semaforo(sem_id);

    println!(
        "Semáforo desbloqueado. Compra completada para el producto: {:?}",
        byte_array_to_string(&registro.producto)
    );
}

pub fn get_shared_memory() -> *mut Registro {
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
    shm_ptr
}

pub fn simular_compra(articulo: *mut Registro, seleccion: i32) {
    unsafe {
        // Modificar directamente el registro en la memoria compartida
        let i_articulo = articulo.add(seleccion as usize);
        let cantidad = (*i_articulo).cantidad;
        if cantidad > 0 {
            (*i_articulo).cantidad -= 1;
        }
        for i in 0..5 {
            let registro = *articulo.add(i);
            println!(
                "Registro {}: {:?}, {:?}, {:?}, {:?}, {:?}",
                i,
                registro.id,
                byte_array_to_string(&registro.producto),
                byte_array_to_string(&registro.marca),
                registro.precio,
                registro.cantidad
            );
        }
    }
}

pub unsafe fn bloq_sem(reg_id: i32) {
    let mut sops: libc::sembuf = libc::sembuf {
        sem_num: 0,
        sem_op: -1,
        sem_flg: 0,
    };
    if semop(reg_id, &mut sops as *mut _, 1) == -1 {
        panic!("Error al bloquear el semáforo");
    }
}

pub unsafe fn desbloquear_semaforo(sem_id: i32) {
    let mut sops: libc::sembuf = libc::sembuf {
        sem_num: 0,
        sem_op: 1,
        sem_flg: 0,
    };

    if semop(sem_id, &mut sops as *mut _, 1) == -1 {
        panic!("Error al desbloquear el semáforo");
    }
}

pub unsafe fn get_sem(identificador: i32) -> i32 {
    let file_key = CString::new(FILE_KEY).expect("CString::new failed");
    let key = ftok(file_key.as_ptr(), identificador);
    if key == -1 {
        panic!("Hubo un error al generar la clave con ftok");
    }
    let sem_id = libc::semget(key, 1, 0);
    if sem_id == -1 {
        panic!("Hubo un error al obtener el semáforo");
    }
    sem_id
}
