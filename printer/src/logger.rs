use std::io::Write;

enum LogWriter {
    DEFAULT(std::io::Stdout),
    FILE(std::fs::File),
}

impl std::io::Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        match self {
            LogWriter::DEFAULT(w) => w.write(buf),
            LogWriter::FILE(w) => w.write(buf),
        }
    }
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        match self {
            LogWriter::DEFAULT(w) => w.flush(),
            LogWriter::FILE(w) => w.flush(),
        }
    }
}

pub struct Logger {
    writer: std::io::BufWriter<LogWriter>,
}

impl Logger {
    pub fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.writer.write(buf)
    }

    pub fn flush_line(&mut self) -> std::result::Result<(), std::io::Error> {
        self.writer.write(b"\n")?;
        self.writer.flush()
    }

    pub fn default() -> Self {
        Self{writer: std::io::BufWriter::new(LogWriter::DEFAULT(std::io::stdout()))}
    }

    pub fn file(path: String) -> Self {
        Self{writer: std::io::BufWriter::new(LogWriter::FILE(std::fs::File::options().write(true).create(true).truncate(true).open(path).unwrap()))}
    }
}
