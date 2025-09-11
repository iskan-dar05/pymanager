use std::fs;
use std::path::Path;



pub fn create_project(name: &str) {
	let path = Path::new(name);
	if path.exists() {
		println!("Project '{}' already exists!", name);
	}
	else{
		fs::create_dir_all(path).expect("Failed to create project directory");
		println!("create new project: {}", name);
	
		// Create pyproject.toml file
		let toml = format!(
			r#"[project]
	name = "{0}"
	version = "0.1.0"
	description = "A Python project managed by pymanager"
	"#,
		name
	);
	fs::write(path.join("pyproject.toml"), toml);
		.expect("Failed to write pyproject.toml");
	}
}
