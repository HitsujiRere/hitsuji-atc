use std::{
    path::{Path, PathBuf},
    process::Command,
};

pub fn compile(code_path: &Path, is_debug: bool) -> Result<PathBuf, String> {
    // code_path: コンパイルするC++ファイルパス
    // is_debug: デバッグコンパイルするか

    if !code_path.exists() {
        return Err(format!(
            "指定されたファイルが存在しません: {}",
            code_path.display()
        ));
    }
    if !code_path.is_file() {
        return Err(format!(
            "指定されたパスはファイルではありません: {}",
            code_path.display()
        ));
    }

    let file_stem = code_path.file_stem().unwrap();
    let file_parent = code_path.parent().unwrap_or(Path::new("."));

    // 実行ファイルパス
    let exec_path = if is_debug {
        let mut new_file_name = file_stem.to_os_string();
        new_file_name.push("-dev");
        file_parent.join(new_file_name)
    } else {
        file_parent.join(file_stem)
    };

    let compiler = "g++";
    let _status = Command::new(compiler)
        .arg(code_path)
        .arg("-o")
        .arg(&exec_path)
        .status()
        .map_err(|err| format!("コンパイルに失敗しました: {}", err))?;

    Ok(exec_path)
}
