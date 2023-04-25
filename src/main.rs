use kweb::webs::WebServer;

fn main() {
    let ws = WebServer::new("0.0.0.0:7878", 4);
    ws.start().expect("web server crashed sad");
    

    println!("shutting down time");
}
