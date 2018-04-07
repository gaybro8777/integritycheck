use std::path::{Path,PathBuf};
use getopts::Options;

pub const USAGE : &'static str = "\
usage: fhistory ack [options] <path>
Acknowledge changes to files in the repository and create a new snapshot

options:
  -d,--data_dir=PATH     Set the path of the repository/data directory
                         default: '.'
  -x,--index_dir=PATH    Set the path of the index directory. Note that this
                         path is relative to the data directory. Absolute
                         paths are allowed. default: '.fh'
  --help                 Print this help message and exit
";

pub fn perform(args: &Vec<String>) -> Result<bool, ::Error> {
  let mut flag_cfg = Options::new();
  flag_cfg.optopt("d", "data_dir", "data_dir", "PATH");
  flag_cfg.optopt("x", "index_dir", "index_dir", "PATH");

  let flags = match flag_cfg.parse(args) {
    Ok(f) => f,
    Err(e) => return Err(e.to_string()),
  };

  let pathspec = match flags.free.get(0) {
    Some(p) => p,
    None => return Err("need a path (e.g. 'fhistory ack .')".into()),
  };

  let data_path = flags.opt_str("data_dir").unwrap_or(::DEFAULT_DATA_DIR.into());
  let index_path = flags.opt_str("index_dir").unwrap_or(::DEFAULT_INDEX_DIR.into());

  println!("[1/4] Loading index...");
  let mut index = ::IndexDirectory::open(&Path::new(&data_path), &Path::new(&index_path))?;
  let mut snapshot = match index.latest() {
    Some(idx) => index.load(&idx)?,
    None => ::IndexSnapshot::new()
  };

  println!("[2/4] Scanning file metadata...");
  let scan_opts = ::index_scan::ScanOptions {
    exclude_paths: vec!(PathBuf::from(&index_path)),
    exclusive_paths: None,
  };

  let mut updates = ::index_scan::scan_metadata(
      &Path::new(&data_path),
      &pathspec,
      &scan_opts)?;

  println!("[3/4] Computing file checksums...");
  updates = ::index_scan::scan_checksums(
      &Path::new(&data_path),
      updates,
      &scan_opts)?;

  println!("[4/4] Committing new snapshot...");
  snapshot.clear(&pathspec);
  snapshot.merge(&updates);
  index.append(&snapshot)?;

  return Ok(true);
}
