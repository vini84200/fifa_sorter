use std::io;
use crate::{DB, Query};
use crate::knowledge::QueryResult;
use crate::reading::initialize;

pub async fn main_loop() {
    println!("Inicializando...");
    let start = std::time::Instant::now();
    let mut db = DB::new();
    initialize(&mut db).await.unwrap();
    let elapsed = start.elapsed();
    println!("Inicializado em {:?}", elapsed);

    loop {
        print!("> ");
        let mut query = String::new();
        io::stdin().read_line(&mut query).unwrap();
        let query = Query::try_from(query);
        match query {
            Ok(query) => {
                let start = std::time::Instant::now();
                let res = db.run_query(query);
                let elapsed = start.elapsed();
                match res {
                    Ok(res) => print_res(res),
                    Err(e) => println!("Error in query execution: {:?}", e),
                }
                println!("Elapsed: {:?}", elapsed);
            }
            Err(e) => {
                println!("Error in parsing: {}", e);
                continue;
            }
        }
        // println!("{:?}", query);

    }
}

fn print_res(res: QueryResult) {
    match res {
        QueryResult::Jogadores(jogadores) => {
            for jogador in jogadores {
                println!("{:?}", jogador);
            }
        },
        QueryResult::Jogador(jogador) => {
            println!("{:?}", jogador);
        },
        QueryResult::User(user) => {
            println!("{:?}", user);
        },
    }
}