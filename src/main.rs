use {
    std::{
        io::Result,
        env, 
        fs::{copy, 
            remove_file,
            create_dir_all,
            remove_dir_all,
            rename
        }, 
        path::{Path, PathBuf},
        // EXPERIMENTO: usando chrono
        // por ser muito mais simples
        /*time::{SystemTime, UNIX_EPOCH}*/},
    chrono::offset::Local
};

fn move_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        create_dir_all(dst)?;

        for entry in src.read_dir()? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                move_dir_recursive(&src_path, &dst_path)?;
            } else {
                rename(&src_path, &dst_path)?;
            }
        }
    } else {
        rename(src, dst)?;
    }

    remove_dir_all(src)?;
    Ok(())
}

fn main() -> Result<()> {
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
            // EXPERIMENTO: usando chrono
            //let date = SystemTime::now()
            //    .duration_since(UNIX_EPOCH)
            //    .expect("Time went backwards")
            //    .as_secs();
            let date_str = format!("{}", 
                           Local::now().format("%Y-%m-%d-%Hh%Mm%Ss"));
            let mut new_filename = source_filename.to_os_string();
            new_filename.push("_");
            new_filename.push(&date_str);
            dest_file = PathBuf::from(&dest_dir);
            dest_file.push(&new_filename);
        }
      
        if source_file.is_dir() {
            match create_dir_all(&dest_file) {
                Ok(_) => { move_dir_recursive(&source_file, &dest_file)?
                    // falta mover o conteúdo!
                    //match remove_dir_all(&source_file) {
                    //    Err(err) => eprintln!("Impossível remover {}.\n{}",
                    //         &source_file.to_string_lossy(),
                    //         err),
                    //    _ => {}
                    //}
                },
                Err(err) => {
                    eprintln!(r#"Não foi possível criar o diretório {}.
                    \n.{}"#,
                    &dest_file.to_string_lossy(),
                    err)}
            }
        } else if source_file.is_file() {
            match copy(&source_file, &dest_file) {
                Ok(_) => { 
                    // TODO: modo verboso
                    //println!("Movido {} para {}", 
                    //        source_file.to_string_lossy(), 
                    //        dest_file.to_string_lossy());
                    match remove_file(&source_file) {
                           Ok(_) => {},
                               // TODO: modo verboso
                               //println!("{} removido com sucesso.", 
                               //&source_file.to_string_lossy()),
                                Err(err) => eprintln!(
                                        "Erro ao remover {}\n{}", 
                                        &source_file.to_string_lossy(), 
                                        err) 
                    }
                },
                Err(err) => eprintln!("Erro ao mover {}: {}", 
                            source_file.to_string_lossy(), 
                            err),
            }
        }
        else { eprintln!(r#"Não foi possível determinar se {:?} é 
               um arquivo ou diretório."#, 
               &source_file.to_string_lossy())}
    }

Ok(())
}
