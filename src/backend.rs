use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Config {
    pub options: Options,
    pub repositories: Vec<Repository>,
}

#[derive(Debug)]
pub struct Options {
    pub root_dir: PathBuf,
    pub db_path: PathBuf,
    pub cache_dirs: Vec<PathBuf>,
    pub hook_dirs: Vec<PathBuf>,
    pub gpg_dir: PathBuf,
    pub log_file: PathBuf,
    pub hold_pkg: Vec<String>,
    pub ignore_pkg: Vec<String>,
    pub ignore_group: Vec<String>,
    pub includes: Vec<PathBuf>,
    pub architecture: Architecture,
    pub xfer_command: String,
    pub no_upgrade: Vec<String>,
    pub no_extract: Vec<String>,
    pub clean_method: CleanMethod,
    pub sig_level: String,
    pub local_file_sig_level: String,
    pub remote_file_sig_level: String,
    pub use_syslog: bool,
    pub color: bool,
    pub no_progress_bar: bool,
    pub check_space: bool,
    pub verbose_pkg_lists: bool,
    pub disable_download_timeout: bool,
    pub parallel_downloads: u32,
    pub download_user: String,
    pub disable_sandbox: bool,
}

#[derive(Debug)]
pub enum Architecture {
    Auto,
    I686,
    X86_64,
}

#[derive(Debug)]
pub enum CleanMethod {
    KeepInstalled,
    KeepCurrent,
}

#[derive(Debug)]
pub struct Repository {
    pub name: String,
    pub servers: Vec<String>,
    pub sig_level: String,
    pub usage: Usage,
    pub cache_server: Option<String>,
}

#[derive(Debug)]
pub enum Usage {
    Sync,
    Search,
    Install,
    Upgrade,
    All,
}

fn parse_bool(value: &str) -> Result<bool, String> {
    match value.to_lowercase().as_str() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(format!("Invalid boolean value: {}", value)),
    }
}

fn parse_architecture(value: &str) -> Result<Architecture, String> {
    match value.to_lowercase().as_str() {
        "auto" => Ok(Architecture::Auto),
        "i686" => Ok(Architecture::I686),
        "x86_64" => Ok(Architecture::X86_64),
        _ => Err(format!("Invalid architecture: {}", value)),
    }
}

fn parse_clean_method(value: &str) -> Result<CleanMethod, String> {
    match value.to_lowercase().as_str() {
        "keepinstalled" => Ok(CleanMethod::KeepInstalled),
        "keepcurrent" => Ok(CleanMethod::KeepCurrent),
        _ => Err(format!("Invalid clean method: {}", value)),
    }
}

fn parse_usage(value: &str) -> Result<Usage, String> {
    match value.to_lowercase().as_str() {
        "sync" => Ok(Usage::Sync),
        "search" => Ok(Usage::Search),
        "install" => Ok(Usage::Install),
        "upgrade" => Ok(Usage::Upgrade),
        "all" => Ok(Usage::All),
        _ => Err(format!("Invalid usage: {}", value)),
    }
}

fn default_options() -> Options {
    Options {
        root_dir: PathBuf::from("/"),
        db_path: PathBuf::from("/var/lib/pacman/"),
        cache_dirs: vec![PathBuf::from("/var/cache/pacman/pkg/")],
        hook_dirs: vec![PathBuf::from("/etc/pacman.d/hooks")],
        gpg_dir: PathBuf::from("/etc/pacman.d/gnupg"),
        log_file: PathBuf::from("/var/log/pacman.log"),
        hold_pkg: vec![],
        ignore_pkg: vec![],
        ignore_group: vec![],
        includes: vec![],
        architecture: Architecture::Auto,
        xfer_command: String::new(),
        no_upgrade: vec![],
        no_extract: vec![],
        clean_method: CleanMethod::KeepInstalled,
        sig_level: "Required DatabaseOptional".to_string(),
        local_file_sig_level: "MD5SUM".to_string(),
        remote_file_sig_level: "MD5SUM".to_string(),
        use_syslog: false,
        color: true,
        no_progress_bar: false,
        check_space: false,
        verbose_pkg_lists: false,
        disable_download_timeout: false,
        parallel_downloads: 5,
        download_user: "nobody".to_string(),
        disable_sandbox: false,
    }
}

pub fn parse_config(path: &str) -> Result<Config, String> {
    let file =
        std::fs::File::open(path).map_err(|e| format!("Failed to open config file: {}", e))?;
    let reader = std::io::BufReader::new(file);

    let mut sections = HashMap::new();
    let mut current_section = None;

    for line_result in reader.lines() {
        let line = line_result.map_err(|e| format!("Failed to read line: {}", e))?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if line.starts_with('[') && line.ends_with(']') {
            current_section = Some(line[1..line.len() - 1].to_string());
            sections
                .entry(current_section.clone().unwrap())
                .or_insert_with(HashMap::new);
        } else if let Some(eq_idx) = line.find('=') {
            let key = line[..eq_idx].trim().to_string();
            let value = line[eq_idx + 1..].trim().to_string();
            if let Some(section) = current_section.as_ref() {
                sections
                    .entry(section.clone())
                    .or_insert_with(HashMap::new)
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(value);
            }
        }
    }

    let binding = HashMap::new();
    let options_section = sections.get("options").unwrap_or(&binding);
    let mut options = default_options();

    if let Some(root_dir) = options_section.get("RootDir").and_then(|v| v.first()) {
        options.root_dir = PathBuf::from(root_dir);
    }
    if let Some(db_path) = options_section.get("DBPath").and_then(|v| v.first()) {
        options.db_path = PathBuf::from(db_path);
    }
    if let Some(gpg_dir) = options_section.get("GPGDir").and_then(|v| v.first()) {
        options.gpg_dir = PathBuf::from(gpg_dir);
    }
    if let Some(log_file) = options_section.get("LogFile").and_then(|v| v.first()) {
        options.log_file = PathBuf::from(log_file);
    }
    if let Some(xfer_command) = options_section.get("XferCommand").and_then(|v| v.first()) {
        options.xfer_command = xfer_command.clone();
    }
    if let Some(sig_level) = options_section.get("SigLevel").and_then(|v| v.first()) {
        options.sig_level = sig_level.clone();
    }
    if let Some(local_file_sig_level) = options_section
        .get("LocalFileSigLevel")
        .and_then(|v| v.first())
    {
        options.local_file_sig_level = local_file_sig_level.clone();
    }
    if let Some(remote_file_sig_level) = options_section
        .get("RemoteFileSigLevel")
        .and_then(|v| v.first())
    {
        options.remote_file_sig_level = remote_file_sig_level.clone();
    }
    if let Some(architecture) = options_section.get("Architecture").and_then(|v| v.first()) {
        options.architecture = parse_architecture(architecture)?;
    }
    if let Some(clean_method) = options_section.get("CleanMethod").and_then(|v| v.first()) {
        options.clean_method = parse_clean_method(clean_method)?;
    }
    if let Some(use_syslog) = options_section.get("UseSyslog").and_then(|v| v.first()) {
        options.use_syslog = parse_bool(use_syslog)?;
    }
    if let Some(color) = options_section.get("Color").and_then(|v| v.first()) {
        options.color = parse_bool(color)?;
    }
    if let Some(no_progress_bar) = options_section.get("NoProgressBar").and_then(|v| v.first()) {
        options.no_progress_bar = parse_bool(no_progress_bar)?;
    }
    if let Some(check_space) = options_section.get("CheckSpace").and_then(|v| v.first()) {
        options.check_space = parse_bool(check_space)?;
    }
    if let Some(verbose_pkg_lists) = options_section
        .get("VerbosePkgLists")
        .and_then(|v| v.first())
    {
        options.verbose_pkg_lists = parse_bool(verbose_pkg_lists)?;
    }
    if let Some(disable_download_timeout) = options_section
        .get("DisableDownloadTimeout")
        .and_then(|v| v.first())
    {
        options.disable_download_timeout = parse_bool(disable_download_timeout)?;
    }
    if let Some(parallel_downloads) = options_section
        .get("ParallelDownloads")
        .and_then(|v| v.first())
    {
        match parallel_downloads.parse::<u32>() {
            Ok(val) => options.parallel_downloads = val,
            Err(e) => return Err(format!("Invalid ParallelDownloads value: {}", e)),
        }
    }
    if let Some(download_user) = options_section.get("DownloadUser").and_then(|v| v.first()) {
        options.download_user = download_user.clone();
    }
    if let Some(disable_sandbox) = options_section
        .get("DisableSandbox")
        .and_then(|v| v.first())
    {
        options.disable_sandbox = parse_bool(disable_sandbox)?;
    }

    if let Some(cache_dirs) = options_section.get("CacheDir") {
        options.cache_dirs = cache_dirs.iter().map(|p| PathBuf::from(p)).collect();
    }
    if let Some(hook_dirs) = options_section.get("HookDir") {
        options.hook_dirs = hook_dirs.iter().map(|p| PathBuf::from(p)).collect();
    }
    if let Some(hold_pkg) = options_section.get("HoldPkg") {
        options.hold_pkg = hold_pkg.clone();
    }
    if let Some(ignore_pkg) = options_section.get("IgnorePkg") {
        options.ignore_pkg = ignore_pkg.clone();
    }
    if let Some(ignore_group) = options_section.get("IgnoreGroup") {
        options.ignore_group = ignore_group.clone();
    }
    if let Some(includes) = options_section.get("Include") {
        options.includes = includes.iter().map(|p| PathBuf::from(p)).collect();
    }
    if let Some(no_upgrade) = options_section.get("NoUpgrade") {
        options.no_upgrade = no_upgrade.clone();
    }
    if let Some(no_extract) = options_section.get("NoExtract") {
        options.no_extract = no_extract.clone();
    }

    let mut repositories = Vec::new();
    for (section_name, section) in &sections {
        if section_name != "options" {
            let mut repo = Repository {
                name: section_name.clone(),
                servers: Vec::new(),
                sig_level: "Required DatabaseOptional".to_string(),
                usage: Usage::Sync,
                cache_server: None,
            };

            if let Some(include_paths) = section.get("Include") {
                for include_path in include_paths {
                    let file = File::open(include_path).map_err(|e| {
                        format!("Failed to open mirrorlist {}: {}", include_path, e)
                    })?;
                    let reader = BufReader::new(file);

                    for mirror_line in reader.lines() {
                        let line = mirror_line
                            .map_err(|e| format!("Failed to read line from mirrorlist: {}", e))?;
                        let line = line.trim();
                        if !line.is_empty() && !line.starts_with('#') {
                            repo.servers.push(line.to_string());
                        }
                    }
                }
            }

            if let Some(servers) = section.get("Server") {
                repo.servers.extend(servers.clone());
            }

            if let Some(sig_level) = section.get("SigLevel").and_then(|v| v.first()) {
                repo.sig_level = sig_level.clone();
            }
            if let Some(usage) = section.get("Usage").and_then(|v| v.first()) {
                repo.usage = parse_usage(usage)?;
            }
            if let Some(cache_server) = section.get("CacheServer").and_then(|v| v.first()) {
                repo.cache_server = Some(cache_server.clone());
            }

            repositories.push(repo);
        }
    }

    Ok(Config {
        options,
        repositories,
    })
}
