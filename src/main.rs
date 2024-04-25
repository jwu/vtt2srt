use clap::{Arg, Command};
use std::collections::HashSet;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, Eq, PartialEq)]
struct SubtitleEntry {
    start: String,
    end: String,
    text: Vec<String>,
}

impl Ord for SubtitleEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.start.clone(), self.end.clone(), self.text.clone()).cmp(&(other.start.clone(), other.end.clone(), other.text.clone()))
    }
}

impl PartialOrd for SubtitleEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for SubtitleEntry {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
        self.text.hash(state);
    }
}

fn vtt_to_srt_timecode(vtt_timecode: &str) -> String {
    vtt_timecode.replace('.', ",")
}

fn parse_vtt_file(file_path: &Path) -> io::Result<HashSet<SubtitleEntry>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut entries = HashSet::new();
    let mut current_text = Vec::new();
    let mut current_start = String::new();
    let mut current_end = String::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains("-->") {
            if !current_text.is_empty() {
                entries.insert(SubtitleEntry {
                    start: vtt_to_srt_timecode(&current_start),
                    end: vtt_to_srt_timecode(&current_end),
                    text: current_text.clone(),
                });
                current_text.clear();
            }
            let parts: Vec<&str> = line.split(" --> ").collect();
            current_start = parts[0].trim().to_string();
            current_end = parts[1].trim().to_string();
        } else if !line.trim().is_empty() && !line.starts_with("NOTE") && !line.starts_with("WEBVTT") && !line.starts_with("X-TIMESTAMP-MAP") {
            current_text.push(line);
        }
    }
    // Add the last entry
    if !current_text.is_empty() {
        entries.insert(SubtitleEntry {
            start: vtt_to_srt_timecode(&current_start),
            end: vtt_to_srt_timecode(&current_end),
            text: current_text,
        });
    }

    Ok(entries)
}

fn merge_vtts_to_srt(folder_path: &str) -> io::Result<()> {
    let paths = fs::read_dir(folder_path)?;
    let mut all_entries = HashSet::new();

    for entry in paths.filter_map(Result::ok) {
        if entry.path().extension().map_or(false, |ext| ext == "vtt") {
            let entries = parse_vtt_file(&entry.path())?;
            all_entries.extend(entries);
        }
    }

    let mut sorted_entries: Vec<_> = all_entries.into_iter().collect();
    sorted_entries.sort(); // Sort entries after deduplication

    let mut output_file = File::create(Path::new(folder_path).join("output.srt"))?;
    for (index, entry) in sorted_entries.iter().enumerate() {
        write!(output_file, "{}\n{} --> {}\n", index + 1, entry.start, entry.end)?;
        for text_line in &entry.text {
            write!(output_file, "{}\n", text_line)?;
        }
        write!(output_file, "\n")?;
    }

    Ok(())
}

fn main() {
    let matches = Command::new("VTT to SRT Converter")
        .version("1.0")
        .author("Your Name <your_email@example.com>")
        .about("Converts VTT files to SRT format, sorts and deduplicates entries.")
        .arg(Arg::new("folder-path")
             .help("The path to the folder containing VTT files")
             .required(true)
             .index(1))
        .get_matches();

    let folder_path = matches.get_one::<String>("folder-path").unwrap();

    if let Err(e) = merge_vtts_to_srt(folder_path) {
        eprintln!("Error: {}", e);
    }
}
