use std::net::TcpListener;

fn main() {
    println!("Serva");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming(){
        let stream = stream.unwrap();

        println!("Connction established!");
    }
}
