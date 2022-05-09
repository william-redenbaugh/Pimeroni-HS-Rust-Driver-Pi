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

fn process_matrix_animation(matrix_string: &mut MatrixAnimationData, matrix: &mut pimeroni_unicornhd::Matrix){
    match(&matrix_string.matrix_animation_state){
        MatrixAnimationTypes::Text=> {
            matrix.print_string(matrix_string.string_text.clone());
        }, 
        MatrixAnimationTypes::FlashRed=> {
            matrix.set_matrix(200, 0, 0);
            thread::sleep(time::Duration::from_millis(300));
            matrix.set_matrix(0, 0, 0);
        }, 
        MatrixAnimationTypes::FlashGreen=> {
            matrix.set_matrix(0, 200, 0);
            thread::sleep(time::Duration::from_millis(300));
            matrix.set_matrix(0, 0, 0);
        },
        MatrixAnimationTypes::FlashBlue=> {
            matrix.set_matrix(0, 0, 200);
            thread::sleep(time::Duration::from_millis(300));
            matrix.set_matrix(0, 0, 0);
        }, 
        MatrixAnimationTypes::FlashWhite=> {
            matrix.set_matrix(90, 90, 90);
            thread::sleep(time::Duration::from_millis(300));
            matrix.set_matrix(0, 0, 0);
        }, 
        MatrixAnimationTypes::SolidRed=> {
            matrix.set_matrix(200, 0, 0);
            thread::sleep(time::Duration::from_millis(300));
        }, 
        MatrixAnimationTypes::SolidGreen=> {
            matrix.set_matrix(0, 200, 0);
            thread::sleep(time::Duration::from_millis(300));
        },
        MatrixAnimationTypes::SolidBlue=> {
            matrix.set_matrix(0, 0, 200);
            thread::sleep(time::Duration::from_millis(300));
        }, 
        MatrixAnimationTypes::SolidWhite=> {
            matrix.set_matrix(90, 90, 90);
            thread::sleep(time::Duration::from_millis(300));
        }
        MatrixAnimationTypes::Empty=> {
            matrix.set_matrix(0, 0, 0);
            thread::sleep(time::Duration::from_millis(300));
        }
    }
}

fn handle_matrix_animation(matrix_rx: Receiver<MatrixAnimationData>){
    let mut matrix: pimeroni_unicornhd::Matrix = pimeroni_unicornhd::matrix_setup(12.0);
    // Clear matrix
    matrix.update(); 

    let mut latest_matrix_animation_data = MatrixAnimationData{
        matrix_animation_state: MatrixAnimationTypes::Text, 
        string_text: String::from("")
    };
    loop {

        match matrix_rx.recv_timeout(Duration::from_millis(300)){
            Ok(data)=>{
                latest_matrix_animation_data = data;
            }
            Err(_e)=>{}
        }
        //println!("Matrix thread recieved string: {}", matrix_string.string_text); 

        process_matrix_animation(&mut latest_matrix_animation_data, &mut matrix);
    }    
}

fn parse_matrix_states(matrix_state_str: String) -> MatrixAnimationTypes{
    let mut matrix_state_enum = MatrixAnimationTypes::Text; 
    match matrix_state_str.as_str(){
        "\"text\"" => matrix_state_enum = MatrixAnimationTypes::Text, 
        "\"flashRed\"" => matrix_state_enum = MatrixAnimationTypes::FlashRed, 
        "\"flashGreen\"" => matrix_state_enum = MatrixAnimationTypes::FlashGreen, 
        "\"flashBlue\"" => matrix_state_enum = MatrixAnimationTypes::FlashBlue, 
        "\"flashWhite\"" => matrix_state_enum = MatrixAnimationTypes::FlashWhite, 
        "\"solidRed\"" => matrix_state_enum = MatrixAnimationTypes::SolidRed, 
        "\"solidGreen\"" => matrix_state_enum = MatrixAnimationTypes::SolidGreen, 
        "\"solidBlue\"" => matrix_state_enum = MatrixAnimationTypes::SolidBlue, 
        "\"solidWhite\"" => matrix_state_enum = MatrixAnimationTypes::SolidWhite, 
        "\"empty\"" => matrix_state_enum = MatrixAnimationTypes::Empty, 
        _ => matrix_state_enum = MatrixAnimationTypes::Empty,
    }
    return matrix_state_enum; 
}

fn handle_client(mut stream: TcpStream, matrix_tx: Sender<MatrixAnimationData>) {
    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);
    let mut read = [0; 1028];
    loop {
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    // Connection was closed.
                    break;
                }
                // Parsed out array for string
                let mut read_vec = vec![0; n];
                for k in 0..n{
                    read_vec[k] = read[k];
                }
                // Parse and package data into string from byterray in JSON format.
                let output_string = String::from_utf8_lossy(&read_vec);
                let data: serde_json::Value = serde_json::from_str(&output_string).unwrap();
                let matrix_state_str = &data["state"].to_string().clone();
                let matrix_state_enum = parse_matrix_states(matrix_state_str.to_string());
                let matrix_print_str = &data["str"];
                let matrix_animation_details = MatrixAnimationData{
                    matrix_animation_state: matrix_state_enum,
                    string_text: matrix_print_str.to_string()
                };
                matrix_tx.send(matrix_animation_details);
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn main() {

    let (matrix_tx, matrix_rx): (Sender<MatrixAnimationData>, Receiver<MatrixAnimationData>) = mpsc::channel();
    let _matrix_animation_handler = thread::spawn(move||{handle_matrix_animation(matrix_rx)});

    let listener = TcpListener::bind("192.168.1.21:2020").unwrap();
    println!("Setup TCP server socket"); 
    loop{
        for stream in listener.incoming() {
            let mut matrix_tx_clone = matrix_tx.clone();
            match stream {
                Ok(stream) => {
                    //println!("Generating Thread to handle communication");
                    thread::spawn(move || {
                        handle_client(stream, matrix_tx_clone);
                    });
                }
                Err(_) => {
                    println!("There was an error with handling communication"); 
                    println!("Error");
                }
            }
        }
    }
}
