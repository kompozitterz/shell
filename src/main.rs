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
            println!("Bye!");
            break;
        }

        // Afficher la coommande (pour test)
        println!("Vous avez tapé: {}", input);
    }
}