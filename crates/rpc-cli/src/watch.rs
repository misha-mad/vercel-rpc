use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Instant;

use anyhow::{Context, Result};
use colored::Colorize;
use notify::RecursiveMode;
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};

use crate::config::RpcConfig;
use crate::{codegen, parser};

/// Runs the watch loop: performs an initial generation, then watches for changes
/// in the api directory and regenerates TypeScript files on each change.
///
/// Blocks until the process receives SIGINT (Ctrl+C).
pub fn run(config: &RpcConfig) -> Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })
    .context("Failed to set Ctrl+C handler")?;

    print_banner(config);

    // Initial generation
    match generate(config) {
        Ok(()) => {}
        Err(e) => print_error(&e),
    }

    // Set up file watcher with debouncing
    let (tx, rx) = mpsc::channel();
    let debounce_duration = std::time::Duration::from_millis(config.watch.debounce_ms);

    let mut debouncer = new_debouncer(debounce_duration, tx)
        .context("Failed to create file watcher")?;

    debouncer
        .watcher()
        .watch(config.input.dir.as_ref(), RecursiveMode::Recursive)
        .with_context(|| format!("Failed to watch {}", config.input.dir.display()))?;

    println!(
        "  {} for changes in {}\n",
        "Watching".cyan().bold(),
        config.input.dir.display().to_string().underline(),
    );

    while running.load(Ordering::SeqCst) {
        match rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(Ok(events)) => {
                let has_rs_change = events.iter().any(|e| {
                    e.kind == DebouncedEventKind::Any
                        && e.path.extension().is_some_and(|ext| ext == "rs")
                });

                if has_rs_change {
                    let changed: Vec<&Path> = events
                        .iter()
                        .filter(|e| e.path.extension().is_some_and(|ext| ext == "rs"))
                        .map(|e| e.path.as_path())
                        .collect();

                    print_change(&changed);

                    match generate(config) {
                        Ok(()) => {}
                        Err(e) => print_error(&e),
                    }
                }
            }
            Ok(Err(errs)) => {
                eprintln!("  {} Watch error: {errs}", "✗".red().bold());
            }
            Err(mpsc::RecvTimeoutError::Timeout) => continue,
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    println!("\n  {} Stopped watching.", "●".dimmed());
    Ok(())
}

/// Performs a full scan + generation cycle, printing timing info.
fn generate(config: &RpcConfig) -> Result<()> {
    let start = Instant::now();

    let manifest = parser::scan_directory(&config.input)?;

    let types_content = codegen::typescript::generate_types_file(
        &manifest,
        config.codegen.preserve_docs,
        config.codegen.naming.fields,
    );
    write_file(&config.output.types, &types_content)?;

    let client_content = codegen::client::generate_client_file(
        &manifest,
        &config.output.imports.types_specifier(),
        config.codegen.preserve_docs,
    );
    write_file(&config.output.client, &client_content)?;

    let elapsed = start.elapsed();
    let proc_count = manifest.procedures.len();
    let struct_count = manifest.structs.len();

    println!(
        "  {} Generated {} procedure(s), {} struct(s) in {:.0?}",
        "✓".green().bold(),
        proc_count.to_string().bold(),
        struct_count.to_string().bold(),
        elapsed,
    );
    println!(
        "    {} {}",
        "→".dimmed(),
        config.output.types.display().to_string().dimmed(),
    );
    println!(
        "    {} {}",
        "→".dimmed(),
        config.output.client.display().to_string().dimmed(),
    );

    Ok(())
}

/// Writes content to a file, creating parent directories as needed.
fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    std::fs::write(path, content)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

fn print_banner(config: &RpcConfig) {
    println!();
    println!(
        "  {} {}",
        "vercel-rpc".bold(),
        "watch mode".cyan(),
    );
    println!(
        "  {} {}",
        "api dir:".dimmed(),
        config.input.dir.display(),
    );
    println!(
        "  {} {}",
        "types:".dimmed(),
        config.output.types.display(),
    );
    println!(
        "  {} {}",
        "client:".dimmed(),
        config.output.client.display(),
    );
    println!();
}

fn print_change(paths: &[&Path]) {
    for p in paths {
        let name = p.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
        println!("\n  {} {}", "↻".yellow().bold(), name);
    }
}

fn print_error(err: &anyhow::Error) {
    eprintln!("  {} {err:#}", "✗".red().bold());
    for cause in err.chain().skip(1) {
        eprintln!("    {} {cause}", "caused by:".dimmed());
    }
}
