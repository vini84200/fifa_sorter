use std::borrow::Cow;
use std::io;
use crate::{DB, Query};
use crate::knowledge::QueryResult;
use crate::reading::initialize;
use reedline::{DefaultPrompt, Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};
use tabled::{Modify, Style, Table, Width, Tabled};
use tabled::object::Segment;
use crate::models::{JogadorComRating, Rating, User};

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
                               Ok(res) => print_res(res, &db),
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

fn print_res(res: QueryResult, db: &DB) {
    match res {
        QueryResult::Jogadores(jogadores) => {
            // Create pager
            show_jogadores(&jogadores);
        },
        QueryResult::Jogador(jogador) => {
            show_jogador(jogador);
        },
        QueryResult::User(user) => {
            show_user(user, db);
        },
    }
}

fn show_jogadores(jogadores: &Vec<JogadorComRating>) {

    if jogadores.is_empty() {
        println!("Nenhum jogador encontrado");
        return;
    }
    let mut page = 1;

    loop {
        let mut table = Table::new(&jogadores[(page - 1) * 20..std::cmp::min(page * 20, jogadores.len())])
            .with(Style::modern())
            .with(Modify::new(Segment::all()).with(Width::wrap(19)));

        println!("{}", table);
        println!("Page {}/{}", page, (jogadores.len() as f32 / 20.0).ceil() as u32);
        println!("Press enter to go to next page, or type a number to go to that page. Type 'q' to quit.");
        let mut line_editor = Reedline::create();
        let prompt = PagerPrompt::new(page,
                                      (jogadores.len() as f32 / 20.0).ceil() as usize);
        let line = line_editor.read_line(&prompt);
        if line.is_err() { break; }
        let line = line.unwrap();
        match line {
            Signal::Success(a) => {
                if a.trim().is_empty() && page < (jogadores.len() as f32 / 20.0).ceil() as usize {
                    page += 1;
                } else if a.trim() == "q"
                {
                    break;
                } else if a.trim() == "p" {
                    if page > 1 {
                        page -= 1;
                    }
                } else {
                    let page_num = a.trim().parse::<usize>();
                    if page_num.is_err() {
                        println!("Invalid page number");
                        continue;
                    }
                    let page_num = page_num.unwrap();
                    if page_num > 0 && page_num <= (jogadores.len() as f32 / 20.0).ceil() as usize {
                        page = page_num;
                    }
                }
            },
            _ => {
                println!("Going back to main menu");
                break;
            }
        }
    }
}

fn show_jogador(jogador: JogadorComRating) {
    println!("Nome: {}", jogador.get_name());
    println!("Id: {}", jogador.get_sofifa_id());
    println!("Posição: {}", jogador.get_pos());
    println!("Rating: {}", jogador.get_rating());
    println!("Avaliações: {}", jogador.get_rating_count());
    println!("Tags:");
    for tag in jogador.get_tags() {
        println!("\t{}", tag);
    }
}

#[derive(Debug, Tabled)]
struct Avaliacao {
    id: u32,
    jogador: String,
    nota: f32,
    nota_geral: f32,
    count: u32,
}

impl Avaliacao {
    fn sort_by_rating(a: &Avaliacao, b: &Avaliacao) -> std::cmp::Ordering {
        a.nota.partial_cmp(&b.nota).unwrap()
    }
    fn sorts(aval: Vec<Avaliacao>) -> Vec<Avaliacao> {
        let mut aval = aval;
        // Implementing shell sort
        let mut gap = aval.len() / 2;
        while gap > 0 {
            for i in gap..aval.len() {
                let mut j = i;
                while j >= gap && Avaliacao::sort_by_rating(&aval[j - gap], &aval[j]) == std::cmp::Ordering::Greater {
                    aval.swap(j, j - gap);
                    j -= gap;
                }
            }
            gap /= 2;
        }
        aval
    }
}

fn show_user(user: User, db: &DB) {
    println!("Avaliações do usuário: {}", user.get_id());
    let avaliacoes = user.get_ratings();
    println!("O usuário avaliou {} jogadores", avaliacoes.len());
    if avaliacoes.len() > 20 {
        println!("Mostrando as 20 primeiras avaliações");
    }
    let ratings = avaliacoes.iter().map(|r| {
        let jogador = db.get_jogador(r.get_sofifa_id());
        let jogador = jogador.unwrap();
        Avaliacao {
            id: r.get_sofifa_id(),
            jogador: jogador.get_name().to_string(),
            nota: r.get_rating(),
            nota_geral: jogador.get_rating(),
            count: jogador.get_rating_count(),
        }
    }).collect::<Vec<Avaliacao>>();
    let mut ratings = Avaliacao::sorts(ratings);
    ratings.reverse();
    let ratings = &ratings[..std::cmp::min(20, ratings.len())];


    let table = Table::new(ratings)
        .with(Style::modern())
        .with(Modify::new(Segment::all()).with(Width::wrap(19)));
    println!("{}", table);
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


struct PagerPrompt {
    page: usize,
    max_page: usize,
}

impl Prompt for PagerPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        return ": ".into();
    }

    fn render_prompt_right(&self) -> Cow<str> {
        return format!("Page {}/{}", self.page, self.max_page).into();
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

impl PagerPrompt {
    fn new(page: usize, max_page: usize) -> Self {
        Self {
            page,
            max_page,
        }
    }
}