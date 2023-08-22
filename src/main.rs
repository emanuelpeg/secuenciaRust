use std::io;

use crate::serie::iniciar_secuencia;

mod serie;

fn main() {
    let mut puntos = 0u8;
    loop {
        let serie:[u16;4] = iniciar_secuencia();

        println!("Serie: ");
        
        for i in 0..2 {
            print!(" {} ", serie[i]);
        }

        println!("_____ {} ", serie[3]);

        println!("Ingrese el valor faltante: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("error: unable to read user input");
        let x = input.trim().parse().expect("Error!! debe ser un numero.");
        
        if serie[2].eq(&x) {
            println!("Ganaste !! ");
            puntos = puntos + 1;
        } else {
            println!("Perdiste !! ");
        }
        println!("Puntos :  {} ", puntos);
        println!("Desea continuar jugando? (y/n): ");
        let mut continuar = String::new();
        io::stdin().read_line(&mut continuar).expect("error: unable to read user input");
        
        if continuar.trim().ne("y"){
            return;
        }
    }
    
    
}
