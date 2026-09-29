use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("Iniciando......");

    // async
    //     let saludo1 = saludar("Juan").await;
    //     println!("{}", saludo1);
    //
    //     let saludo2 = saludar("Pedro").await;
    //     println!("{}", saludo2);

    // join
    //     let (saludo1, saludo2, _) = tokio::join!(saludar("Juan"), saludar("Pedro"), progreso());
    //
    //     println!("{}", saludo1);
    //     println!("{}", saludo2);

    let tarea_saludo = tokio::spawn(saludar("Juan"));
    let tarea_progreso = tokio::spawn(progreso());

    let saludo_recibido = match tarea_saludo.await {
        Ok(saludo) => saludo,
        Err(e) => panic!("Error: {}", e),
    };

    if let Err(e) = tarea_progreso.await {
        panic!("Error: {}", e);
    }

    println!("{}", saludo_recibido);

    println!("Finalizado.");
}

async fn saludar(value: &str) -> String {
    sleep(Duration::from_secs(3)).await;
    format!("Hola, {}", value)
}

async fn progreso() {
    for i in 1..=3 {
        sleep(Duration::from_secs(1)).await;
        println!("Progreso: {}/3", i);
    }
}
