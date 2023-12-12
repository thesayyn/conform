use anyhow::{anyhow, Context, Ok};
use std::fs;
use std::io::{self, Read, Write};
use std::process::{Child, Command, Stdio};

use crate::test_case::TestCase;

pub struct Runner {
    command: Command,
    child: Option<Child>,
}



impl Runner {
    pub fn new(program: &String) -> Self {
        Self {
            command: Command::new(&program),
            child: None,
        }
    }
    

    pub fn set_env(&mut self, k: String, v: String) {
        self.command.env(k, v);
    }

    pub fn set_env_all(&mut self, env: Vec<(String, String)>) {
        for (k, v) in env.iter() {
            self.command.env(k, v);
        }
    }

    pub fn set_stderr(&mut self, stderr: String) -> anyhow::Result<()> {
        let stderr = match stderr.as_str() {
            "" | "ignore" => Stdio::null(),
            _ => {
                let file = fs::File::create(stderr).with_context(|| "failed to create stderr file for the runner")?;
                Stdio::from(file)
            },
        };
        
        self.command.stderr(stderr);
        Ok(())
    }

    pub fn spawn(&mut self) -> anyhow::Result<&mut Self> {
        if self.child.is_some() {
            return Err(anyhow!("program is already running"));
        }
    
        let child = self
            .command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .with_context(|| "failed to spawn the testee program")?;
        
        self.child = Some(child);
        Ok(self)
    }

    pub fn send_case(&mut self, case: &TestCase) -> anyhow::Result<Vec<u8>> {
        if self.child.is_none() {
            return Err(anyhow!("program is not running"));
        }
        let child = self.child.as_mut().unwrap();
        let stdin = child.stdin.as_mut().unwrap();
        let stdout = child.stdout.as_mut().unwrap();

        let plen = case.payload.len() as u32;
        stdin
            .write(&plen.to_le_bytes())
            .with_context(|| "failed to write payload length to stdin")?;
        stdin
            .write(&case.payload)
            .with_context(|| "failed to write payload to stdin")?;

        let mut rlen = [0u8; 4];
        stdout
            .read_exact(&mut rlen)
            .with_context(|| "failed to read response length from stdout")?;

        let mut r = vec![0u8; u32::from_le_bytes(rlen) as usize];

        stdout
            .read_exact(&mut r)
            .with_context(|| "failed to read response from stdout")?;
        Ok(r)
    }

    pub fn kill(&mut self) -> anyhow::Result<(), io::Error> {
        self.child.as_mut().unwrap().kill()
    }
}
