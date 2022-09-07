use std::borrow::Cow;
use std::io;
use crate::{DB, Query};
use crate::knowledge::QueryResult;
use crate::reading::initialize;
use reedline::{DefaultPrompt, Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};

pub async fn main_loop() {
    println!("Inicializando...");
    let start = std::time::Instant::now();
    let mut db = DB::new();
    initialize(&mut db).await.unwrap();
    let elapsed = start.elapsed();
    println!("Inicializado em {:?}", elapsed);

    let mut line_editor = Reedline::create();
    let prompt = CleanPrompt::default();
    loop {
        let line = line_editor.read_line(&prompt);
           match line {
               Ok(Signal::Success(buffer)) => {
                   if buffer.trim().is_empty() { continue; }

                   if buffer.trim() == "exit" || buffer.trim() == "quit" {
                       break;
                   }
                   let query = Query::try_from(buffer);
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
               },
               Ok(Signal::CtrlC) => {
                   println!("Ctrl-C");
                   break;
               },
               x => {
                   println!("Error: {:?}", x);
                   break;
               }
            // println!("{:?}", query);
        }
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

#[derive(Default)]
struct CleanPrompt;

impl Prompt for CleanPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        return ">> ".into();
    }

    fn render_prompt_right(&self) -> Cow<str> {
        return "".into();
    }

    fn render_prompt_indicator(&self, prompt_mode: PromptEditMode) -> Cow<str> {
        return " ".into();
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        return ":MLT ".into();
    }

    fn render_prompt_history_search_indicator(&self, history_search: PromptHistorySearch) -> Cow<str> {
        return "HIUS".into();
    }
}