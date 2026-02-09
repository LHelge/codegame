fn main() {
    let script_path = std::path::Path::new("assets/default_ai.lua");
    match std::fs::read_to_string(script_path) {
        Ok(code) => {
            println!("Loaded AI script from {}", script_path.display());
            snake::run_with_script(&code);
        }
        Err(e) => {
            eprintln!(
                "Could not load {}: {e} — running without AI",
                script_path.display()
            );
            snake::run();
        }
    }
}
