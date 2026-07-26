use std::env;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    // FIX: Server passes the binary path name as the second argument ("core").
    // We check if any arguments were passed before exiting.
    if args.len() < 2 {
        std::process::exit(1);
    }

    if &args[1] == "core" {
        let mut stdout = io::stdout();
        let mut stderr = io::stderr();
        let mut buffer = [0u8; 4096];

        // Connect to the shared Unix domain socket file
        let mut stream = match UnixStream::connect("/tmp/yom_core.sock") {
            Ok(s) => s,
            Err(e) => {
                let _ = writeln!(stderr, "failed to connect to socket: {}", e);
                std::process::exit(1);
            }
        };

        loop {
            // Render the shell prompt line
            let _ = write!(stdout, "$ ");
            let _ = stdout.flush();

            let mut input = String::new();
            if std::io::stdin().read_line(&mut input)? == 0 {
                break; // Exit cleanly on standard input EOF (Ctrl+D)
            }

            let trimmed = input.trim();
            if trimmed.is_empty() {
                continue; // Skip empty inputs without hitting the server
            } else if trimmed == "exit" {
                std::process::exit(0);
            } else if trimmed.starts_with("exit ") {
                let code: i32 = trimmed.strip_prefix("exit ").unwrap().parse().unwrap();
                std::process::exit(code);
            }

            // Format the message with a newline delimiter so the server reads it properly
            let payload = format!("eval {}\n", trimmed);
            if stream.write_all(payload.as_bytes()).is_err() {
                let _ = writeln!(stderr, "failed to write to server");
                break;
            }

            // Block execution here until the server finishes evaluation and sends a token back
            match stream.read(&mut buffer) {
                Ok(0) => {
                    let _ = writeln!(stderr, "error: FATAL: CLIENT CLOSED CONNECTION");
                    break; // Server terminated or closed the socket
                }
                Ok(bytes_read) => {
                    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let token = response.trim();

                    // Process the ultra-minimal protocol markers
                    if token.starts_with(':') {
                        let new_dir = token.strip_prefix(':').unwrap();
                        if !new_dir.is_empty() {
                            let _ = env::set_current_dir(new_dir);
                        }
                    } else if token == "C" {
                        // Received clear confirmation token. Ready for next user prompt loop.
                    }
                }
                Err(e) => {
                    let _ = writeln!(stderr, "error: failed to read response: {}", e);
                    break;
                }
            }
        }
    } else if &args[1] == "eval" {
        let line = &args[2];
        if line == "c-hook" {
            println!("c-hook detected!");
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    Ok(())
}
