use std::env;
use std::io::{self, Write};

fn main() {
    loop {
        // Afficher le prompt 
        print!("$ ");
        io::stdout().flush().unwrap();

        // Lire l'entrée utilisateur 
        let mut input = String::new();
        if let Err (_) = io::stdin().read_line(&mut input) {
            println!("Erreur lors de la lecture de l'entrée. Veuillez ressayez s'il vous plait!");
            continue;
        }

        // Supprimer les espaces superflus 
        let input = input.trim();

        // Vérifier la commmande 'exit' pour quitter le shell 
        if input == "exit" {
            // println!("Bye!");
            break;
        }

        // Séparer la commande et les arguments
        let mut parts = input.split_whitespace();
        if let Some(command) = parts.next() {
            match command {
                "echo" => {
                    // Récuperer le reste de la ligne après 'echo'
                    let args: Vec<&str> = parts.collect();
                    println!("{}", args.join(" "));
                }
                "pwd" => {
                    // Obtenir le répertoire courant 
                    match env::current_dir() {
                        Ok(path) => println!("{}", path.display()),
                        Err(err) => eprintln!("Erreur : impossible de récuperer le répertoire courant ({})", err),
                    }
                }
                _ => {
                    // Commande non reconnue 
                    println!("Commande '{}' introuvable", command)
                }
            }
        }

        // Afficher la coommande (pour test)
        // println!("Vous avez tapé: {}", input);
    }
}