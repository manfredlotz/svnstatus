use std::process::Command;


pub struct CmdReturn {
    pub status: bool,
    pub output: Option<String>,
}

// Runs a command in a shell
// If the command returns 0 stdout gets captured. Otherwise
// stderr gets captured and returned.
pub fn run_cmd(cmd: &str, argument: &[&str]) -> CmdReturn {
    let output = Command::new(cmd)
        .args(argument.iter())
        .output()
        .unwrap_or_else(|_| panic!("Failed to execute process `{}`", cmd));

    if output.status.success() {
        CmdReturn {
            status: true,
            output: Some(String::from_utf8_lossy(&output.stdout).to_string()),
        }
    } else {
        CmdReturn {
            status: false,
            output: Some(String::from_utf8_lossy(&output.stderr).to_string()),
        }
    }
}


pub struct SvnStatus {
    added: u32,
    conflicted: u32,
    deleted: u32,
    ignored: u32,
    modified: u32,
    replaced: u32,
    unknown: u32,
    missing: u32,
}

impl SvnStatus {
    pub fn new() -> SvnStatus {
        SvnStatus{
            added:0,
            conflicted: 0,
            deleted: 0,
            ignored: 0,
            modified: 0,
            replaced:0,
            unknown: 0,
            missing: 0
        }

    }
    pub fn analyze_svn_status(&mut self, status: &str) {
        for line in status.lines() {
            let first_char = line.chars().next().unwrap();
            match first_char {
                'A' => self.added += 1,
                'C' => self.conflicted += 1,
                'D' => self.deleted += 1,
                'I' => self.ignored += 1,
                'M' => self.modified += 1,
                'R' => self.replaced += 1,
                '?' => self.unknown += 1,
                '!' => self.missing += 1,
                _ => (),
            }
        }
    }

    pub fn get_status(&self) -> String {
        let mut stat: String = String::new();
        stat.push_str(&self.get_string('A', self.added));
        stat.push_str(&self.get_string('C', self.conflicted));
        stat.push_str(&self.get_string('D', self.deleted));
        stat.push_str(&self.get_string('I', self.ignored));
        stat.push_str(&self.get_string('M', self.modified));
        stat.push_str(&self.get_string('R', self.replaced));
        stat.push_str(&self.get_string('?', self.unknown));
        stat.push_str(&self.get_string('!', self.missing));

        stat
    }
    pub fn get_string(&self, first: char, count: u32) -> String {
        match count {
            0 => String::new(),
            1 => String::from(first),
            n => format!("{}:{}", first, n)
        }

    }

}


impl Default for SvnStatus {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    let rc = run_cmd("svn", &["--non-interactive",  "info", "--no-newline", "--show-item", "revision"]);

    if !rc.status {
        std::process::exit(1);
    }

    let mut revision = String::from("") ;
    if let Some(out) = rc.output {
        revision = out;
    }

    let rc = run_cmd("svn", &["--non-interactive",  "status"]);
    if !rc.status {
        std::process::exit(1);
    }
    let mut status_info = String::from("") ;
    if let Some(out) = rc.output {
        status_info = out;
    }
    if status_info.is_empty() {
        //println!("[{}]", &revision);
        // leave out outer [] parenthesises
        println!("{}", &revision);
        std::process::exit(0);
    }
    let mut svn_status = SvnStatus::new();
    svn_status.analyze_svn_status(&status_info);
    let status_string = svn_status.get_status();

    //println!("[{} {}]", &revision, &status_string);
    // leave out outer [] parenthesises
    println!("{} {}", &revision, &status_string);

}
