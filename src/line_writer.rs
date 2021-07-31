use std::future::Future;
use std::pin::Pin;

pub struct LineWriter {
    pub state: String,
}

impl LineWriter {
    pub async fn start_writing_lines<Return, M>(&mut self, f: M) -> Return
    where
        M: for<'b> FnOnce(&'b mut Self) -> Pin<Box<dyn Future<Output = Return> + 'b>>,
    {
        let r = f(self).await;

        self.state = "written".to_string();

        r
    }

    pub async fn write_newline(&mut self) -> String {
        // do something
        println!("{:?}", self.state);
        self.state = "start".to_string();
        "foo bar".to_string()
    }
}
