use termion::{color::{self}, style};
use chrono::{self, DateTime, Local};
use std::{process, path::Path, fs::File, fmt};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    INFO,
    DEBUG, 
    WARNING,
    ERROR,
    DONE,
    FATAL
}

impl fmt::Display for LogLevel{

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogLevel::INFO => write!(f,"INFO"),
            LogLevel::DEBUG => write!(f,"DEBUG"),
            LogLevel::WARNING => write!(f,"WARNING"),
            LogLevel::ERROR => write!(f,"ERROR"),
            LogLevel::FATAL => write!(f,"FATAL"),
            LogLevel::DONE => write!(f,"DONE"),
        }

    }
}

pub struct Log{
    path : String,
    extension : String,
    service : String, 
    separation_char : String
}

impl Log {
    pub fn new(p: &str, e: &str, s: &str, c: &str) -> Log{
        Log{path: String::from(p),extension: String::from(e), service: String::from(s), separation_char : String::from(c)}
    }

    pub fn info(&self,log: &str){
        let log_entity = LogEntity::new(chrono::offset::Local::now(), LogLevel::INFO, log,self.separation_char.clone(), &self.create_log_path());
        self.processing(log_entity);
    }

    pub fn warning(&self,log: &str){
        let log_entity = LogEntity::new(chrono::offset::Local::now(), LogLevel::WARNING, log,self.separation_char.clone(), &self.create_log_path());
        self.processing(log_entity);
    }

    pub fn error(&self,log: &str){
        let log_entity = LogEntity::new(chrono::offset::Local::now(), LogLevel::ERROR, log,self.separation_char.clone(), &self.create_log_path());
        self.processing(log_entity);
    }

    pub fn fatal(&self,log: &str){
        let log_entity = LogEntity::new(chrono::offset::Local::now(), LogLevel::FATAL, log,self.separation_char.clone(), &self.create_log_path());
        self.processing(log_entity);
    }
    
    pub fn debug(&self,log: &str){
        let log_entity = LogEntity::new(chrono::offset::Local::now(), LogLevel::DEBUG, log,self.separation_char.clone(), &self.create_log_path());
        self.processing(log_entity);
    }
    
    pub fn done(&self,log: &str){
        let log_entity = LogEntity::new(chrono::offset::Local::now(), LogLevel::DONE, log,self.separation_char.clone(), &self.create_log_path());
        self.processing(log_entity);
    }
    

    fn processing(&self,log_e: LogEntity){      
        log_e.print();
        log_e.write();
    }

    fn create_log_path(&self) -> String{
        let last_char = self.path.chars().last().unwrap();
        let mut log_path = self.path.clone();
        if last_char != '/'{
            log_path = log_path + "/";
        }
        
        log_path + &self.service+ &self.extension

    }
}

pub struct LogEntity{
    date_time : DateTime<Local>,
    log_level : LogLevel,
    log_message : String, 
    char : String,
    path : String
}

impl LogEntity{
    pub fn new(dt: DateTime<Local>, ll:LogLevel,lm: &str, c : String, p: &str ) -> LogEntity{
        LogEntity{ date_time: dt, log_level: ll, log_message: String::from(lm), char: c, path: String::from(p)}
    }

    pub fn print(&self){
        let date_time = self.date_time.format("%Y-%m-%d %H:%M:%S");
        match self.log_level {
            LogLevel::INFO => {
                println!("{}{}[{}{}{}]\t[{}INFO\t{}] {}", style::Bold, color::Fg(color::White),
                           color::Fg(color::LightBlack), date_time, color::Fg(color::White), 
                           color::Fg(color::Blue), color::Fg(color::White), self.log_message);
            },
            LogLevel::DEBUG => {
                println!("{}{}[{}{}{}]\t[{}DEBUG\t{}] {}", style::Bold, color::Fg(color::White),
                color::Fg(color::LightBlack), date_time, color::Fg(color::White), 
                color::Fg(color::Magenta), color::Fg(color::White), self.log_message);
            },
            LogLevel::WARNING => {
                println!("{}{}[{}{}{}]\t[{}WARNING{}] {}", style::Bold, color::Fg(color::White),
                color::Fg(color::LightBlack), date_time, color::Fg(color::White), 
                color::Fg(color::Yellow), color::Fg(color::White), self.log_message);
            },
            LogLevel::ERROR => {
                println!("{}{}[{}{}{}]\t[{}ERROR\t{}] {}", style::Bold, color::Fg(color::White),
                color::Fg(color::LightBlack), date_time, color::Fg(color::White), 
                color::Fg(color::LightRed), color::Fg(color::White), self.log_message);

            },
            LogLevel::FATAL => {
                println!("{}{}[{}{}{}]\t[{}FATAL\t{}] {}", style::Bold, color::Fg(color::White),
                color::Fg(color::LightBlack), date_time, color::Fg(color::White), 
                color::Fg(color::Red), color::Fg(color::White), self.log_message);
                process::exit(1);
            },
            LogLevel::DONE => {
                println!("{}{}[{}{}{}]\t[{}DONE\t{}] {}", style::Bold, color::Fg(color::White),
                color::Fg(color::LightBlack), date_time, color::Fg(color::White), 
                color::Fg(color::Green), color::Fg(color::White), self.log_message);
            },
        }
    }

    pub fn write(&self){
        let file_ok = OpenOptions::new().append(true).open(self.path.clone());
        let mut file : File;
        let is_error = file_ok.is_err();
        if is_error {
            let path = Path::new(&self.path);
            file = File::create(&path).unwrap();
            if let Err(_err) = writeln!(file,"{}{}{}{}{}", self.date_time.format("%Y-%m-%d %H:%M:%S"), self.char, self.log_level.to_string(),self.char, self.log_message) {
                println!("Failed to perform necessary steps");
            }

        }else{
            file = file_ok.unwrap();
            if let Err(_err) = writeln!(file,"{}{}{}{}{}", self.date_time.format("%Y-%m-%d %H:%M:%S"), self.char, self.log_level.to_string(),self.char, self.log_message) {
                println!("Failed to perform necessary steps");
            }
        }
    }

}

