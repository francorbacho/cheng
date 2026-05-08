use std::net::Ipv4Addr;

use cheng::{Board, FromIntoFen};
use tiny_http::{Response, Server};

pub fn start_http_server(port: u16) -> Result<(), String> {
    let addr = std::net::SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let server = Server::http(&addr).map_err(|e| format!("Failed to bind to port {port}: {e}"))?;

    log::info!("Listening on http://localhost:{}/uci", port);

    for mut request in server.incoming_requests() {
        let path = request.url();
        log::info!("{} {}", request.method(), path);
        
        // Handle CORS preflight
        if *request.method() == tiny_http::Method::Options {
            let response = Response::from_string("")
                .with_header(cors_header(b"Access-Control-Allow-Origin", b"*"))
                .with_header(cors_header(b"Access-Control-Allow-Methods", b"GET, POST, OPTIONS"))
                .with_header(cors_header(b"Access-Control-Allow-Headers", b"Content-Type"));
            let _ = request.respond(response);
            continue;
        }
        
        // Only handle /uci path
        if !path.starts_with("/uci") {
            let response = Response::from_string("404 Not Found")
                .with_status_code(404)
                .with_header(content_type("text/plain"));
            let _ = request.respond(response);
            continue;
        }

        // Handle GET request with ?fen=...
        if *request.method() == tiny_http::Method::Get {
            if let Some(fen) = get_fen_from_query(path) {
                log::info!("GET FEN={}", fen);
                let board = Board::from_fen(&fen).unwrap_or_default();
                let result = franfish::go(&board);
                let json = format!(r#"{{"movement":"{}"}}"#, result.movement);
                log::info!("GET response={}", json);
                let response = Response::from_string(json)
                    .with_header(content_type("application/json"))
                    .with_header(cors_header(b"Access-Control-Allow-Origin", b"*"));
                let _ = request.respond(response);
                continue;
            }
        }

        // Handle POST request with UCI commands
        // Read the request body
        let mut body = String::new();
        if let Err(e) = request.as_reader().read_to_string(&mut body) {
            let response = Response::from_string(format!("Error reading request: {}", e))
                .with_status_code(400)
                .with_header(content_type("text/plain"));
            let _ = request.respond(response);
            continue;
        }

        // Parse UCI commands from body
        let response = handle_uci_request(&body);
        
        let http_response = Response::from_string(response.clone())
            .with_header(content_type("text/plain"))
            .with_header(cors_header(b"Access-Control-Allow-Origin", b"*"));
        
        let _ = request.respond(http_response);
        log::info!("POST response={}", response.lines().next().unwrap_or(""));
    }

    Ok(())
}

fn content_type(t: &str) -> tiny_http::Header {
    tiny_http::Header::from_bytes(&b"Content-Type"[..], t.as_bytes()).unwrap()
}

fn cors_header(name: &[u8], value: &[u8]) -> tiny_http::Header {
    tiny_http::Header::from_bytes(name, value).unwrap()
}

fn get_fen_from_query(url: &str) -> Option<String> {
    // Parse /uci?fen=...
    let query_start = url.find('?')?;
    let params = &url[query_start + 1..];
    
    for param in params.split('&') {
        if let Some(fen_value) = param.strip_prefix("fen=") {
            // Convert URL-encoded FEN back to normal FEN
            let fen = fen_value.replace('_', "/").replace('+', " ");
            return Some(fen);
        }
    }
    None
}

fn handle_uci_request(body: &str) -> String {
    let mut board = Board::default();
    let mut response = String::new();
    
    for line in body.lines() {
        let cmd: Vec<&str> = line.split_whitespace().collect();
        if cmd.is_empty() {
            continue;
        }
        
        match cmd[0] {
            "position" => {
                if cmd.len() >= 2 {
                    if cmd[1] == "startpos" {
                        board = Board::default();
                    } else if cmd[1] == "fen" {
                        let fen_parts: Vec<&str> = cmd[2..].iter().take(6).cloned().collect();
                        let fen = fen_parts.join(" ");
                        if let Ok(b) = Board::from_fen(&fen) {
                            board = b;
                        }
                    }
                    
                    // Handle moves
                    if let Some(moves_idx) = cmd.iter().position(|&x| x == "moves") {
                        for mv in cmd[(moves_idx + 1)..].iter() {
                            let _ = board.try_feed(*mv);
                        }
                    }
                }
            }
            "go" => {
                // Run franfish search
                let result = franfish::go(&board);
                response.push_str(&format!("bestmove {}\n", result.movement));
            }
            "ucinewgame" => {
                board = Board::default();
            }
            "uci" => {
                response.push_str("uciok\n");
            }
            "isready" => {
                response.push_str("readyok\n");
            }
            _ => {}
        }
    }
    
    if response.is_empty() {
        response.push_str("readyok\n");
    }
    
    response
}