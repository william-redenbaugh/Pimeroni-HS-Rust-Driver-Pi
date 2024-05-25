//use std::{thread, time};
mod pimeroni_unicornhd;
use std::net::{TcpListener, TcpStream};
use std::time; 
use std::thread;
use std::io::Read;
use std::io::Write;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::time::Duration;
use std::net::UdpSocket;

enum MatrixAnimationTypes{
    Text, 
    FlashRed, 
    FlashGreen,
    FlashBlue, 
    FlashWhite, 
    SolidRed, 
    SolidGreen,
    SolidBlue, 
    SolidWhite, 
    Empty,
}
struct MatrixAnimationData{
    matrix_animation_state: MatrixAnimationTypes,
    string_text: String, 
}

fn handle_incoming_message(data_in: &[u8], matrix: &mut pimeroni_unicornhd::Matrix, pixel_index: &mut u32){
    let x_index = data_in[0];
    let y_index = data_in[1];
    let len = data_in[2];

    if x_index == 0{
        for data_index in 0..len{
            *pixel_index = *pixel_index + 1;
            let arr_index = data_index as usize * 3 + 3;
            matrix.set_pixel(*pixel_index / 16, 
                *pixel_index %16,
                data_in[arr_index], 
                data_in[arr_index + 1], 
                data_in[arr_index + 2]);
        }
    }

    if(x_index == 1){
        *pixel_index = 0;
        for data_index in 0..len{
            *pixel_index = *pixel_index + 1;
            let arr_index = data_index as usize * 3 + 3;
            matrix.set_pixel(*pixel_index / 16, 
                *pixel_index %16,
                data_in[arr_index], 
                data_in[arr_index + 1], 
                data_in[arr_index + 2]);
        }
    }

    if x_index == 2{
        for data_index in 0..len{
            *pixel_index = *pixel_index + 1;
            let arr_index = data_index as usize * 3 + 3;
            matrix.set_pixel(*pixel_index / 16, 
                *pixel_index %16,
                data_in[arr_index], 
                data_in[arr_index + 1], 
                data_in[arr_index + 2]);
        }
        matrix.update();
    }

    matrix.update();
}


fn handle_client(mut stream: UdpSocket, matrix_tx: Sender<MatrixAnimationData>) {
    let mut matrix: pimeroni_unicornhd::Matrix = pimeroni_unicornhd::matrix_setup(12.0);
    
    // Clear matrix
    matrix.update(); 

    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);
    let mut read = [0; 1028];
    let mut pixel_index = 0;
    loop {
        match stream.recv_from(&mut read) {
            Ok(n) => {
                handle_incoming_message(&mut read, &mut matrix, &mut pixel_index)
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}


fn main() {
    let (matrix_tx, matrix_rx): (Sender<MatrixAnimationData>, Receiver<MatrixAnimationData>) = mpsc::channel();
    let socket = UdpSocket::bind("192.168.3.249:34254").unwrap();
    
    let mut matrix_tx_clone = matrix_tx.clone();
    handle_client(socket, matrix_tx_clone);
}
