use std::fs;
use std::path::Path;
use std::process::Command;

pub fn create_venv(path: &str, python_path: &str) -> std::io::Result<()> {
    let venv_path = Path::new(path);

    // 1. Create base folders
    fs::create_dir_all(venv_path.join("bin"))?;
    fs::create_dir_all(venv_path.join("lib"))?;

    // 2. Copy or symlink python executable
    let target = venv_path.join("bin/python");
    fs::copy(python_path, &target)?; // symlink would be faster

    // 3. Write pyvenv.cfg
    let cfg = format!(
        "home = {}\ninclude-system-site-packages = false\nversion = 3.11\n",
        python_path
    );
    fs::write(venv_path.join("pyvenv.cfg"), cfg)?;

    Ok(())
}


