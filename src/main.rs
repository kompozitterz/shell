use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::MetadataExt; // Pour accéder aux métadonnées Unix

fn main() {
    loop {
        // Afficher le prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Lire l'entrée utilisateur
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            println!("Erreur lors de la lecture de l'entrée. Réessayez.");
            continue;
        }

        // Supprimer les espaces superflus
        let input = input.trim();

        // Vérifier la commande 'exit' pour quitter le shell
        if input == "exit" {
            println!("Bye!");
            break;
        }

        // Séparer la commande et les arguments
        let mut parts = input.split_whitespace();
        if let Some(command) = parts.next() {
            match command {
                "echo" => {
                    let args: Vec<&str> = parts.collect();
                    println!("{}", args.join(" "));
                }
                "pwd" => {
                    match env::current_dir() {
                        Ok(path) => println!("{}", path.display()),
                        Err(err) => eprintln!("Erreur : impossible de récupérer le répertoire courant ({})", err),
                    }
                }
                "cd" => {
                    if let Some(dir) = parts.next() {
                        if let Err(err) = env::set_current_dir(dir) {
                            eprintln!("Erreur : impossible de changer le répertoire ({})", err);
                        }
                    } else {
                        eprintln!("Erreur : aucun répertoire spécifié.");
                    }
                }
                "ls" => {
                    // Récupérer les options et le répertoire cible
                    let mut show_all = false;
                    let mut long_format = false;
                    let mut append_indicator = false;
                    let mut target_dir = ".";

                    for arg in parts {
                        match arg {
                            "-a" => show_all = true,
                            "-l" => long_format = true,
                            "-F" => append_indicator = true,
                            _ => target_dir = arg, // Si ce n'est pas une option, c'est le répertoire cible
                        }
                    }

                    // Lire le contenu du répertoire
                    match fs::read_dir(target_dir) {
                        Ok(entries) => {
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    let file_name = entry.file_name();
                                    let file_name = file_name.to_string_lossy();

                                    // Fichiers cachés
                                    if !show_all && file_name.starts_with('.') {
                                        continue;
                                    }

                                    if long_format {
                                        // Afficher les détails du fichier
                                        if let Ok(metadata) = entry.metadata() {
                                            let permissions = metadata.permissions();
                                            let file_size = metadata.len();
                                            let modified = metadata.mtime();
                                            println!(
                                                "{:?} {:>10} {:>10} {}",
                                                permissions,
                                                file_size,
                                                modified,
                                                file_name
                                            );
                                        }
                                    } else {
                                        // Format simple
                                        if append_indicator && entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                                            print!("{}/ ", file_name);
                                        } else {
                                            print!("{} ", file_name);
                                        }
                                    }
                                }
                            }
                            println!(); // Nouvelle ligne après la liste
                        }
                        Err(err) => {
                            eprintln!("Erreur : impossible de lire le répertoire ({})", err);
                        }
                    }
                }
                "cat" => {
                    // Récuperer l'argument (non du fichier)
                    if let Some(file_name) = parts.next() {
                        // Lire et afficher le contenu du fichier 
                        match fs::read_to_string(file_name) {
                            Ok(content ) => print!("{}", content),
                            Err(err) => eprint!("Erreur : impossible de lire le fichier ({})", err),
                        }
                    } else {
                        eprint!("Erreur : aucun fichier spécifié.");
                    }
                }
                "cp" => {
                    let source = parts.next();
                    let destination = parts.next();

                    match (source, destination) {
                        (Some(src), Some(dest)) => {
                            if let Err(err) = fs::copy(src, dest) {
                                if err.kind() == std::io::ErrorKind::PermissionDenied {
                                    eprintln!("Erreur : permissions insuffisantes pour copier le fichier.");
                                } else if err.to_string().contains("Read-only file system") {
                                    eprintln!("Erreur : système de fichiers en lecture seule.");
                                } else {
                                    eprintln!("Erreur : impossible de copier le fichier ({})", err);
                                }
                            }                            
                        }
                        _ => {
                            eprintln!("Erreur : vous devez spécifier une source et une destination.");
                        }
                    }
                }
                "rm" => {
                    let mut args: Vec<&str> = parts.collect();
                                
                    if args.is_empty() {
                        eprintln!("Erreur : aucun fichier ou répertoire spécifié.");
                    } else {
                        let recursive = args.contains(&"-r");
                        let target = if recursive {
                            args.retain(|&x| x != "-r");
                            args.get(0)
                        } else {
                            args.get(0)
                        };
                    
                        if let Some(&target) = target {
                            if recursive {
                                if let Err(err) = fs::remove_dir_all(target) {
                                    eprintln!("Erreur : impossible de supprimer le répertoire ({})", err);
                                } else {
                                    println!("Répertoire '{}' supprimé.", target);
                                }
                            } else {
                                if let Err(err) = fs::remove_file(target) {
                                    eprintln!("Erreur : impossible de supprimer le fichier ({})", err);
                                } else {
                                    println!("Fichier '{}' supprimé.", target);
                                }
                            }
                        } else {
                            eprintln!("Erreur : aucun fichier ou répertoire spécifié.");
                        }
                    }
                }
                "mv" => {
                    let source = parts.next();
                    let destination = parts.next();

                    match (source, destination) {
                        (Some(src), Some(dest)) => {
                            if let Err(err) = fs::rename(src, dest) {
                                eprintln!("Erreur : impossible de déplacer ou renommer ({})", err);
                            } else {
                                println!("'{}' déplacé ou renommé en '{}'", src, dest);
                            }
                        }
                        _ => {
                            eprintln!("Erreur : vous devez spécifier une source et une destination.");
                        }
                    }
                }

                _ => {
                    println!("Commande '{}' introuvable", command);
                }
            }
        }
        // Afficher la coommande (pour test)
        // println!("Vous avez tapé: {}", input);
    }
}
