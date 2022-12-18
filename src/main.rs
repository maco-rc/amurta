mod amurta;


fn main() {
    let flag: Vec<&str> = vec!["l"];
    amurta::Commands::find_command("docker", flag )
}