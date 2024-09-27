extern crate libc;
use csv::ReaderBuilder;
use libc::{ftok, shmat, shmget, IPC_CREAT};

use std::ffi::CString;
use std::fs::File;
use std::io::BufReader;
use std::ptr;

use Tarea04::models::Registro;

const FILE_KEY: &str = "/bin/cat";
const SHM_KEY: i32 = 9;

fn string_to_byte_array(s: &str) -> [u8; 50] {
    let mut array = [0u8; 50];
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate().take(50) {
        array[i] = byte;
    }
    array
}

fn byte_array_to_string(array: &[u8; 50]) -> String {
    let mut s = String::new();
    for &byte in array.iter() {
        if byte == 0 {
            break;
        }
        s.push(byte as char);
    }
    s
}

fn read_file(ruta: &str) -> Vec<Registro> {
    let file = File::open(ruta).expect("No se pudo abrir el archivo");
    let reader = BufReader::new(file);

    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(reader);
    let mut registros = Vec::new();

    for result in csv_reader.records() {
        let record = result.expect("Error al leer el registro");

        let id: i32 = record[0].parse().expect("ID inválido");
        let producto = string_to_byte_array(&record[1]);
        let marca = string_to_byte_array(&record[2]);
        let precio: i32 = record[3].parse().expect("Precio inválido");
        let cantidad: i32 = record[4].parse().expect("Cantidad inválida");

        let registro = Registro::new(id, producto, marca, precio, cantidad);
        registros.push(registro);
    }
    registros
}

pub fn set_shared_memory() {
    let file_key = CString::new(FILE_KEY).expect("CString::new failed");

    let key = unsafe { ftok(file_key.as_ptr(), SHM_KEY) };
    if key == -1 {
        panic!("Hubo un error al generar la clave con ftok");
    }
    println!("{}", key);
    let smh_id = unsafe { shmget(key, std::mem::size_of::<[Registro; 5]>(), IPC_CREAT | 0o666) };
    if smh_id == -1 {
        panic!("Hubo un error al crear la memoria compartida");
    }
    let shm_ptr = unsafe { shmat(smh_id, ptr::null(), 0) } as *mut Registro;
    if shm_ptr.is_null() {
        panic!("No se pudo acceder a la memoria compartida");
    }
    let registros = read_file("src/registros.txt");

    for (i, registro) in registros.iter().enumerate() {
        unsafe {
            *shm_ptr.add(i) = *registro;
            let smid = crear_sem(registro.id);
            print!("Sem id: {}", smid);
        }
    }
    unsafe {
        for i in 0..5 {
            let registro = *shm_ptr.add(i);
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

unsafe fn crear_sem(identificador: i32) -> i32 {
    let file_key = CString::new(FILE_KEY).expect("CString::new failed");
    let key = ftok(file_key.as_ptr(), identificador);
    if key == -1 {
        panic!("Hubo un error al generar la clave con ftok");
    }
    let sem_id = libc::semget(key, 1, libc::IPC_CREAT | 0o666);
    if sem_id == -1 {
        panic!("Hubo un error al crear el semáforo");
    }
    //Init sem
    if libc::semctl(sem_id, 0, libc::SETVAL, 1) == -1 {
        panic!("Hubo un error al inicializar el semáforo");
    }
    sem_id
}

fn main() {
    set_shared_memory();
}
