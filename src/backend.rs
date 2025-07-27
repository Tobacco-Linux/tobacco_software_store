use std::{collections::HashSet, path::PathBuf};

use alpm::{SigLevel, Usage};

/*
 * "inspired" by https://github.com/manjaro/libpamac ;)
 */
pub struct AlpmRepo  {
    pub name: String,
    pub siglevel: SigLevel,
    pub siglevel_mask: SigLevel,
    pub usage: Usage,
    pub urls: Vec<String>,
}

impl AlpmRepo {
    pub fn new(name: &str) -> Self {
        AlpmRepo {
            name: name.to_string(),
            siglevel: SigLevel::USE_DEFAULT,
            siglevel_mask: SigLevel::empty(),
            usage: Usage::empty(),
            urls: Vec::new(),
        }
    }

    pub fn equal_name(&self, a: &AlpmRepo, b: &AlpmRepo) -> bool {
        a.name == b.name
    }
}

pub struct AlpmConfig {
    conf_path: PathBuf,
    rootdir: Option<PathBuf>,
    dbpath: Option<PathBuf>,
    logfile: Option<PathBuf>,
    gpgdir: Option<PathBuf>,
    download_user: String,
    disable_sandbox: bool,
    usesyslog: i32,
    checkspace: bool,
    architectures: Vec<String>,
    cachedirs: Vec<PathBuf>,
    hookdirs: Vec<PathBuf>,
    ignoregroups: Vec<String>,
    ignorepkgs: HashSet<String>,
    noextracts: Vec<String>,
    noupgrades: Vec<String>,
    holdpkgs: HashSet<String>,
    syncfirsts: HashSet<String>,
    siglevel: SigLevel,
    localfilesiglevel: SigLevel,
    remotefilesiglevel: SigLevel,
    siglevel_mask: SigLevel,
    localfilesiglevel_mask: SigLevel,
    remotefilesiglevel_mask: SigLevel,
    repo_order: Vec<AlpmRepo>,
}
