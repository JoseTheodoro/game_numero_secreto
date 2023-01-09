use rand::Rng;
use std::io;
use std::cmp::Ordering;

struct Player {
    name: String,
    pontos: u32,
    ativo: bool,
}

impl std::fmt::Debug for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(name: {}, pontos: {}, ativo: {})", self.name, self.pontos, self.ativo)
    }
}

fn main() {
    println!("Bem vindo ao jogo!");

    let mut count = 0u8;
    let mut name_player1 = String::new();
    let mut name_player2 = String::new();

    println!("Escolha o nome do Player1:");
    io::stdin().read_line(&mut name_player1).expect("error");
    println!("Escolha o nome do Player2:");
    io::stdin().read_line(&mut name_player2).expect("error");
    let name_player1 = name_player1.trim();
    let name_player2 = name_player2.trim();

    let mut p1 = Player::cria_player(name_player1.to_string());
    let mut p2 = Player::cria_player(name_player2.to_string());

    println!();
    println!();
    println!("Iniciando o game...");
    println!();

    let numero_secreto:u32 = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let i:u8 = count % 2;

        if i == 0 {
            println!("É a vez do {}", p1.name);
            p1.troca_jogador(&mut p2);
        } else {
            println!("É a vez do {}", p2.name);
            p2.troca_jogador(&mut p1);       
        }

        count += 1;
        let mut numero = String::new();
        
        io::stdin().read_line(&mut numero).expect("error");

        let numero = numero.trim().parse::<u32>().unwrap();
        
            match numero.cmp(&numero_secreto) {
                Ordering::Less => println!("Menor"),
                Ordering::Greater => println!("Maior"),
                Ordering::Equal => {
                    println!("Você Ganhou");
                    break;
                }
            }
    }
}

impl Player {
    fn cria_player(name: String) -> Player {
        let p = Player {
            name: name,
            pontos: 0,
            ativo: false
        };
        p
    }
    fn troca_jogador(&mut self, p: &mut Player) {
        self.ativo = true;
        p.ativo = false;
    }
}