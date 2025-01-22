use clap::Parser;
use notify::{event::ModifyKind, recommended_watcher, Event, EventKind, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc, sync::Arc, sync::atomic::{AtomicBool, Ordering}, time::{Duration, Instant}, fs};
use ctrlc;

const DEBOUNCE_DURATION: Duration = Duration::from_millis(500);

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// path to file to watch
    #[arg(short, long)]
    file: String,

    /// destination path for the new parser
    #[arg(short = 'p', long, default_value = "")]
    gen_path: String,

    /// name for the new parser
    #[arg(short = 'n', long)]
    gen_name: String,
}

/// function to regenerate the parser
/// you'll need to convert to a Path::new()
fn regen_p(gen_path: String, gen_name: String) {
    todo!();
}

fn watch(file: String, gen_path: String, gen_name: String, running: Arc<AtomicBool>) -> Result<()> {
    println!("[watcher] watching file --> {}", file);

    // event channel
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    // check if the file exists in the current running directory
    // but also handle some lazy typing of directory separators
    let current_dir = std::env::current_dir()?;
    let file_path = current_dir.join(file.replace("/", &std::path::MAIN_SEPARATOR.to_string()).replace("\\", &std::path::MAIN_SEPARATOR.to_string()));

    let file_path = if file_path.exists() {
        file_path
    } else {
        Path::new(&file).to_path_buf()
    };

    // create watcher
    let mut watcher = recommended_watcher(tx)?;
    watcher.watch(Path::new(&file), RecursiveMode::NonRecursive)?;

    let mut last_event_time = Instant::now();

    while running.load(Ordering::SeqCst) {
        // non-blocking receive to check termination flag
        match rx.recv_timeout(Duration::from_secs(1)) {
            Ok(event) => match event {
                Ok(event) => {
                    if event.kind == EventKind::Modify(ModifyKind::Any) {
                        // platforms will have different event file operation sequences, so we debounce..
                        // i.e ignore events too close in time
                        // idk how often modifications are expected to happen
                        // so i took this out as a easy-to-modify "contant"
                        // but if this causes problems altogether, just remove the if statement
                        if last_event_time.elapsed() > DEBOUNCE_DURATION {
                            println!("[watcher] modify event detected");
                            last_event_time = Instant::now();

                            // regenerate parser or other logic..
                            // regen_p(gen_path.clone(), gen_name.clone());
                        }
                    }
                }
                Err(e) => println!("[watcher] error --> {:?}", e),
            },
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // check termination flag and continue
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                println!("[watcher] channel disconnected, exiting...");
                break;
            }
        }
    }

    println!("[watcher] exiting...");
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // shared atomic flag to handle SIGINT termination
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    // termination handler
    ctrlc::set_handler(move || {
        println!("\n[watcher] received Ctrl+C...");
        r.store(false, Ordering::SeqCst);
    }).expect("[watcher] error setting Ctrl+C handler");

    // watch the file
    watch(args.file, args.gen_path, args.gen_name, running)
}
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_watch() {
        let test_file_path = "test_file.txt";
        let gen_path = "gen_path";
        let gen_name = "gen_name";

        fs::write(test_file_path, "initial content").unwrap();

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();

        let handle = thread::spawn(move || {
            watch(
                test_file_path.to_string(),
                gen_path.to_string(),
                gen_name.to_string(),
                r,
            )
            .unwrap();
        });

        thread::sleep(Duration::from_secs(1));

        // trigger an event
        fs::write(test_file_path, "modified content").unwrap();

        // wait for the watcher to detect the change
        thread::sleep(Duration::from_secs(2));

        running.store(false, Ordering::SeqCst);
        handle.join().unwrap();
        fs::remove_file(test_file_path).unwrap();
    }
}
