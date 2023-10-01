use std::{
    env, 
    fs::{copy, remove_file/*rename, File, Write*/, create_dir_all}, 
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let files: Vec<String> = env::args().collect();
    if files.len() < 2 {
        eprintln!("Uso: {} <file1> <file2> ...", files[0]);
        std::process::exit(1);
    }

    let dest_dir = match env::var("LIXEIRA") {
        Ok(val) => {
            val
        },
        Err(_) => {
            eprintln!("Erro: A variável $LIXEIRA não está definida.");
            std::process::exit(1);
        }
    };

    if !Path::new(&dest_dir).exists() {
        if let Err(err) = create_dir_all(&dest_dir) {
            eprintln!("Erro ao criar a lixeira em \"{}\"", err);
            std::process::exit(1);
        }
    }

    for file in &files[1..] {
        let source_file = Path::new(&file);
        let mut dest_file = PathBuf::from(&dest_dir);
        let source_filename = Path::new(&file)
            .file_name()
            .expect("Invalid source file path");
        dest_file.push(&source_filename);

        while dest_file.exists() {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();
            let timestamp_str = format!("{}", timestamp);
            let mut new_filename = source_filename.to_os_string();
            new_filename.push("_");
            new_filename.push(&timestamp_str);
            dest_file = PathBuf::from(&dest_dir);
            dest_file.push(&new_filename);
        }

        match copy(&source_file, &dest_file) {
            Ok(_) => { println!("Movido {} para {}", 
                            source_file.to_string_lossy(), 
                            dest_file.to_string_lossy());
                       match remove_file(&source_file) {
                            Ok(_) => println!("{} removido com sucesso.", &source_file.to_string_lossy()),
                            Err(err) => eprintln!("Erro ao remover {}\n{}", &source_file.to_string_lossy(), err) 
                       }
            },
            Err(err) => eprintln!("Erro ao mover {}: {}", 
                            source_file.to_string_lossy(), 
                            err),
        }
    }
}
