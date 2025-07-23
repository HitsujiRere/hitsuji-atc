use indicatif::ProgressBar;
use std::{
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

pub fn compile(code_path: &Path, is_debug: bool) -> Result<PathBuf, ()> {
    // code_path: コンパイルするC++ファイルパス
    // is_debug: デバッグコンパイルするか

    if !code_path.exists() {
        println!("指定されたファイルが存在しません: {}", code_path.display());
        return Err(());
    }
    if !code_path.is_file() {
        println!(
            "指定されたパスはファイルではありません: {}",
            code_path.display()
        );
        return Err(());
    }

    // 実行ファイルパス
    let exec_path = get_exec_path(code_path, is_debug);

    if !should_update(code_path, &exec_path) {
        return Ok(exec_path);
    }

    let bar = ProgressBar::new_spinner().with_message(format!(
        "コンパイル中: {} -> {}",
        code_path.display(),
        exec_path.display()
    ));
    bar.enable_steady_tick(Duration::from_millis(100));

    let output = Command::new("g++")
        .arg(code_path)
        .args([Path::new("-o"), &exec_path])
        .arg("-fdiagnostics-color=always")
        .output();

    bar.finish();

    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(exec_path)
            } else {
                let stderr = String::from_utf8(output.stderr).unwrap();
                println!("コンパイルに失敗しました:");
                println!("{}", stderr);
                Err(())
            }
        }
        Err(err) => {
            println!("コンパイルに失敗しました: {}", err);
            Err(())
        }
    }
}

fn get_exec_path(code_path: &Path, is_debug: bool) -> PathBuf {
    let file_stem = code_path.file_stem().unwrap();
    let file_parent = code_path.parent().unwrap_or(Path::new("."));

    if is_debug {
        let mut new_file_name = file_stem.to_os_string();
        new_file_name.push("-dev");
        file_parent.join(new_file_name)
    } else {
        file_parent.join(file_stem)
    }
}

fn should_update(code_path: &Path, exec_path: &Path) -> bool {
    if !code_path.exists() || !exec_path.exists() {
        return true;
    }

    let code_modified = code_path.metadata().unwrap().modified().unwrap();
    let exec_modified = exec_path.metadata().unwrap().modified().unwrap();

    println!("{:?}", code_modified);
    println!("{:?}", exec_modified);

    code_modified > exec_modified
}
