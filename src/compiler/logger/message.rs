use super::{Log, LogType};

#[derive(Debug)]
pub struct LogMessage {
    pub path: Option<String>,
    pub row: usize,
    pub col: usize,
    pub code: Option<String>,
    pub metadata: Option<String>,
    pub message: String,
    pub comment: Option<String>
}

impl LogMessage {
    pub fn new (path: Option<String>, row: usize, col: usize) -> Self {
        LogMessage {
            row, col, path,
            code: None,
            metadata: None,
            message: format!("<Insert message>"),
            comment: None
        }
    }

    /// Attach code so that error handler can display snippet
    pub fn attach_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// Attach metadata just to give additional information to the programmer handling this message
    pub fn attach_metadata(mut self, metadata: String) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Attach message required to be displayed
    pub fn attach_message<T: AsRef<str>>(mut self, message: T) -> Self {
        self.message = message.as_ref().to_string();
        self
    }

    /// Attach message in case if more details about the error can be provided
    pub fn attach_comment<T: AsRef<str>>(mut self, comment: T) -> Self {
        self.comment = Some(comment.as_ref().to_string());
        self
    }

    /// Commit a default message using Log
    fn commit(&mut self, kind: LogType) {
        let path = match self.path.clone() {
            Some(path) => path,
            None => format!("[code]")
        };
        let mut log = Log::new(path, self.row, self.col, kind)
            .attach_message(self.message.clone());
        if let Some(comment) = self.comment.clone() {
            log = log.attach_comment(comment);
        }
        if let Some(code) = self.code.clone() {
            log = log.attach_code(code);
        }
        log.show().exit();
    }

    /// Show error message using default error handler
    pub fn show_error(&mut self) {
        self.commit(LogType::Error)
    }

    /// Show error message using default warning handler
    pub fn show_warning(&mut self) {
        self.commit(LogType::Warning)
    }

    /// Show error message using default info handler
    pub fn show_info(&mut self) {
        self.commit(LogType::Info)
    }
}