

fn main(){
    let dia_de_la_semana = "lunes";

    match dia_de_la_semana {
        "lunes" | "martes" | "miércoles" | "jueves" | "viernes" => {
            println!("Es un día laboral");
        }
        "sábado" | "domingo" => {
            println!("Es fin de semana");
        }
        _ => {
            println!("Día no reconocido");
        }
    }
    for numero in 1..6 {
        println!("Número: {}", numero);
    }
    let mut contador = 0;

    loop {
        println!("Contador: {}", contador);
        contador += 1;

        if contador >= 5 {
            break;
        }
    }
}