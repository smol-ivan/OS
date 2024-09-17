use fltk::draw::Region;
use libc::{ftok, shmat, shmget};
use raw_sync::locks::*;
use shared_memory::*;
use std::fs::File;
use std::io::Read;
use std::simd::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::{i64, thread};
use tokio::sync::{Semaphore, SemaphorePermit};

use crate::models::*;

fn get_registros_txt() -> Vec<Vec<&str>> {
    let mut registros = Vec::new();
    let mut file = File::open("./src/registros.txt").unwrap();
    let mut contenido = String::new();
    file.read_to_string(&mut contenido).unwrap();
    for linea in contenido.lines().skip(1) {
        let registro: Vec<&str> = linea.split(",").collect();
        registros.push(registro);
    }
    registros
}

fn set_shared_memory() {
    let filename = "/tmp";
    unsafe {
        if let key = ftok(&filename, 1) == -1 {
            panic!("ftok failed");
        }
        if let shmid = shmget(key, 1024, 0777) == -1 {
            panic!("shmget failed");
        }
        if let ptr = shmat(shmid, ptr::null(), 0) == -1 {
            panic!("shmat failed");
        }
    }
}

fn test_shared_memory() {}
