use git_activity_dashboard::GitHubClient;

fn main() {
    let client = GitHubClient::new().expect("Failed to create client");

    let repos = client.list_repos(None).expect("Failed to list repos");

    println!("# Repository Summaries\n");

    for repo in repos.iter().take(40) {
        if repo.fork {
            continue;
        }

        print!("Fetching {}... ", repo.name);

        match client.get_readme("fabiantax", &repo.name) {
            Ok(readme) if !readme.is_empty() => {
                println!("OK");
                // Get first paragraph or heading
                let summary = extract_summary(&readme);
                println!("## {}\n", repo.name);
                println!("**Language:** {}\n", repo.language.as_deref().unwrap_or("N/A"));
                println!("{}\n", summary);
                println!("---\n");
            }
            Ok(_) => println!("(no README)"),
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn extract_summary(readme: &str) -> String {
    let lines: Vec<&str> = readme.lines().collect();
    let mut summary = String::new();
    let mut started = false;

    for line in lines {
        // Skip title
        if line.starts_with("# ") && !started {
            continue;
        }

        // Skip badges and empty lines at start
        if !started && (line.is_empty() || line.contains("![") || line.contains("[!")) {
            continue;
        }

        started = true;

        // Stop at next heading or after 3 lines
        if line.starts_with("## ") || summary.lines().count() >= 3 {
            break;
        }

        if !line.is_empty() {
            summary.push_str(line);
            summary.push('\n');
        }
    }

    summary.trim().to_string()
}
