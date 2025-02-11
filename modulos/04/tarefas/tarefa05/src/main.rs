mod it;
use it::Network;

fn main() {
  let router = it::Router {
    ip: "192.168.0.11".to_string(),
  };

  let ping = router.ping("https://www.udemy.com");
  let traceroute = router.traceroute("https://www.udemy.com");
  let nslookup = router.nslookup("https://www.udemy.com");

  println!("Ping: {}", ping);
  println!("Traceroute: {:?}", traceroute);
  println!("Nslookup: {}", nslookup);
}
