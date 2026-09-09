use std::env;
use std::process::{Command, ExitCode};

#[derive(Debug, Default, PartialEq, Eq)]
struct Options {
    message: Option<String>,
    branch: Option<String>,
    dry_run: bool,
    help: bool,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("gatm: {error}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<(), String> {
    let options = parse_args(env::args().skip(1))?;
    if options.help {
        println!("{}", usage());
        return Ok(());
    }
    if options.dry_run {
        return dry_run(&options);
    }

    let status_output = git(&["status", "--porcelain"])?;
    let files = changed_files(&status_output);
    if files.is_empty() {
        println!("nothing to commit");
        return Ok(());
    }

    let message = options
        .message
        .clone()
        .unwrap_or_else(|| deterministic_message(&files));

    if let Some(branch) = &options.branch {
        git_checked(&["checkout", "-b", branch])?;
    }

    let mut add_args = vec!["add", "--all", "--"];
    add_args.extend(files.iter().map(String::as_str));
    git_checked(&add_args)?;
    git_checked(&["commit", "-m", &message])?;

    let branch = current_branch()?;
    match git_checked(&["push", "-u", "origin", &branch]) {
        Ok(_) => Ok(()),
        Err(error) if error.contains("non-fast-forward") || error.contains("rejected") => {
            Err("run git pull first".to_string())
        }
        Err(error) => Err(error),
    }
}

fn dry_run(options: &Options) -> Result<(), String> {
    let status_output = git(&["status", "--porcelain"])?;
    let files = changed_files(&status_output);
    if files.is_empty() {
        println!("nothing to commit");
        return Ok(());
    }

    let message = options
        .message
        .as_deref()
        .map(str::to_owned)
        .unwrap_or_else(|| deterministic_message(&files));
    println!("changed files:");
    for file in &files {
        println!("  {file}");
    }
    println!("commit message: {message}");
    match &options.branch {
        Some(branch) => println!("branch: create and switch to {branch}"),
        None => println!("branch: current branch"),
    }
    println!("dry run: no changes made");
    Ok(())
}

fn parse_args<I>(args: I) -> Result<Options, String>
where
    I: IntoIterator<Item = String>,
{
    let args: Vec<String> = args.into_iter().collect();
    let mut options = Options::default();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "-d" | "--dry-run" => options.dry_run = true,
            "-m" | "--message" => {
                index += 1;
                options.message = Some(value_for(&args, index, "message")?);
            }
            "-b" | "--branch" => {
                index += 1;
                options.branch = Some(value_for(&args, index, "branch")?);
            }
            "-h" | "--help" => {
                options.help = true;
            }
            flag if flag.starts_with('-') => {
                return Err(format!("unknown option '{flag}'\n\n{}", usage()))
            }
            value => return Err(format!("unexpected argument '{value}'\n\n{}", usage())),
        }
        index += 1;
    }
    Ok(options)
}

fn value_for(args: &[String], index: usize, name: &str) -> Result<String, String> {
    args.get(index)
        .filter(|value| !value.is_empty() && !value.starts_with('-'))
        .cloned()
        .ok_or_else(|| format!("missing {name}\n\n{}", usage()))
}

fn usage() -> &'static str {
    "Usage: gatm [-m <message>] [-b <branch>] [-d]\n\n  -m, --message <message>  Use a custom commit message\n  -b, --branch <branch>    Create and switch to a branch\n  -d, --dry-run            Show the plan without changing Git"
}

fn changed_files(status: &str) -> Vec<String> {
    let mut files: Vec<String> = status
        .lines()
        .filter_map(|line| line.get(3..))
        .map(|path| match path.rsplit_once(" -> ") {
            Some((_, destination)) => destination.to_string(),
            None => path.to_string(),
        })
        .filter(|path| !path.is_empty())
        .collect();
    files.sort();
    files.dedup();
    files
}

fn deterministic_message(files: &[String]) -> String {
    format!("chore: update {}", files.join(", "))
}

fn current_branch() -> Result<String, String> {
    let branch = git(&["branch", "--show-current"])?;
    let branch = branch.trim();
    if branch.is_empty() {
        return Err("could not determine the current branch".to_string());
    }
    Ok(branch.to_string())
}

fn git(args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .map_err(|error| format!("could not run git: {error}"))?;
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        let error = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(if error.is_empty() {
            format!("git {} failed", args.join(" "))
        } else {
            error
        })
    }
}

fn git_checked(args: &[&str]) -> Result<String, String> {
    git(args)
}

#[cfg(test)]
mod tests {
    use super::{changed_files, deterministic_message, parse_args, Options};

    #[test]
    fn extracts_and_sorts_paths_from_porcelain_status() {
        let files = changed_files(" M zed.rs\n?? src/main.rs\nR  old.rs -> new.rs\n");
        assert_eq!(files, ["new.rs", "src/main.rs", "zed.rs"]);
    }

    #[test]
    fn generates_a_stable_message_from_sorted_paths() {
        let files = vec!["a.txt".to_string(), "b.txt".to_string()];
        assert_eq!(deterministic_message(&files), "chore: update a.txt, b.txt");
    }

    #[test]
    fn parses_supported_options() {
        let options = parse_args([
            "-m".to_string(),
            "ship it".to_string(),
            "-b".to_string(),
            "feature/test".to_string(),
            "-d".to_string(),
        ])
        .unwrap();
        assert_eq!(
            options,
            Options {
                message: Some("ship it".to_string()),
                branch: Some("feature/test".to_string()),
                dry_run: true,
                help: false,
            }
        );
    }
}
