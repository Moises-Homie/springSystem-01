use std::io::{self, Write};
use std::process::Command;

enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path + content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let status = Command::new("ls")
                .arg(path)
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => println!("Failed to execute ls"),
            }
        }

        FileOperation::Display(path) => {
            let status = Command::new("cat")
                .arg(path)
                .status();

            match status {
                Ok(s) if s.success() => {}
                _ => println!("Failed to display file"),
            }
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);

            let status = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("File '{}' created successfully.", path);
                }
                _ => println!("Failed to create file"),
            }
        }

        FileOperation::Remove(path) => {
            let status = Command::new("rm")
                .arg(&path)
                .status();

            match status {
                Ok(s) if s.success() => {
                    println!("File '{}' removed successfully.", path);
                }
                _ => println!("Failed to remove file"),
            }
        }

        FileOperation::Pwd => {
            let status = Command::new("pwd").status();

            match status {
                Ok(s) if s.success() => {}
                _ => println!("Failed to print working directory"),
            }
        }
    }
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        print!("Enter your choice (0-5): ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        let choice = choice.trim();

        match choice {
            "1" => {
                print!("Enter directory path: ");
                io::stdout().flush().unwrap();

                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                perform_operation(FileOperation::List(path.trim().to_string()));
            }

            "2" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();

                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                perform_operation(FileOperation::Display(path.trim().to_string()));
            }

            "3" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();

                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                print!("Enter content: ");
                io::stdout().flush().unwrap();

                let mut content = String::new();
                io::stdin().read_line(&mut content).unwrap();

                perform_operation(FileOperation::Create(
                    path.trim().to_string(),
                    content.trim().to_string(),
                ));
            }

            "4" => {
                print!("Enter file path: ");
                io::stdout().flush().unwrap();

                let mut path = String::new();
                io::stdin().read_line(&mut path).unwrap();

                perform_operation(FileOperation::Remove(path.trim().to_string()));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
