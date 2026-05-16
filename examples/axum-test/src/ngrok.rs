use std::{
    env,
    process::{Child, Command, Stdio},
};

pub struct NgrokProcessGuard {
    child: Child,
}

impl Drop for NgrokProcessGuard {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

pub fn start_ngrok_tunnel(
    port: u16,
) -> Result<Option<NgrokProcessGuard>, Box<dyn std::error::Error>> {
    if !read_env_bool("NGROK_ENABLED", false) {
        return Ok(None);
    }

    let mut command = Command::new("ngrok");
    command
        .arg("http")
        .arg(port.to_string())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .stdin(Stdio::null());

    let ngrok_domain = env::var("NGROK_DOMAIN")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    if let Some(domain) = &ngrok_domain {
        let ngrok_domain_flag = domain
            .strip_prefix("https://")
            .or_else(|| domain.strip_prefix("http://"))
            .unwrap_or(domain)
            .trim_end_matches('/')
            .to_string();

        command.arg(format!("--domain={ngrok_domain_flag}"));
        println!("ngrok tunnel domain configured: {domain}");
    } else {
        println!(
            "ngrok enabled without NGROK_DOMAIN (ephemeral URL). Visit ngrok dashboard/terminal output to copy the tunnel URL."
        );
    }

    if let Ok(token) = env::var("NGROK_AUTHTOKEN") {
        if !token.trim().is_empty() {
            command.arg(format!("--authtoken={token}"));
        }
    }

    let child = command.spawn().map_err(|error| {
        format!(
            "failed to start ngrok. Ensure ngrok is installed and in PATH. underlying error: {error}"
        )
    })?;

    println!("ngrok process started");
    Ok(Some(NgrokProcessGuard { child }))
}

fn read_env_bool(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => default,
    }
}
