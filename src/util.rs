//! machinectl model — manage systemd-nspawn containers/VMs.

#[derive(Debug, Clone)]
pub struct Machine {
    pub name: String,
    pub state: String,
    pub image: String,
    pub pid: u32,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub machines: Vec<Machine>,
}

impl Default for Model {
    fn default() -> Self { Self { machines: vec![
        Machine { name: "fedora".into(), state: "running".into(), image: "/var/lib/machines/fedora".into(), pid: 1234 },
    ] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn list(&self) -> &[Machine] { &self.machines }

    pub fn start(&mut self, name: &str) -> bool {
        if let Some(m) = self.machines.iter_mut().find(|m| m.name == name) {
            m.state = "running".into(); true
        } else { false }
    }

    pub fn stop(&mut self, name: &str) -> bool {
        if let Some(m) = self.machines.iter_mut().find(|m| m.name == name) {
            m.state = "stopped".into(); true
        } else { false }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start_stop() {
        let mut m = Model::new();
        assert!(m.start("fedora"));
        assert!(m.stop("fedora"));
        assert_eq!(m.list()[0].state, "stopped");
    }
}
