pub struct Router {
  pub ip: String,
}

pub trait Network {
  fn ping(&self, host: &str) -> bool;
  fn traceroute(&self, host: &str) -> Vec<String>;
  fn nslookup(&self, host: &str) -> String;
}

impl Network for Router {
  fn ping(&self, host: &str) -> bool {
    println!("Pinging {} from {}", host, self.ip);
    return true;
  }

  fn traceroute(&self, host: &str) -> Vec<String> {
    println!("Tracing route to {} from {}", host, self.ip);
    return vec![
      "192.168.1.1".to_string(),
      "10.0.0.1".to_string(),
      "172.16.0.1".to_string(),
      host.to_string(),
    ];
  }

  fn nslookup(&self, host: &str) -> String {
    println!("Looking up DNS for {} from {}", host, self.ip);
    return "1.1.1.1".to_string();
  }
}
