#!/usr/bin/python3

from http.server import HTTPServer, SimpleHTTPRequestHandler
import json
import subprocess
from urllib.parse import urlparse, parse_qs

class CORSRequestHandler(SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory="./web", **kwargs)

    def end_headers(self):
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET')
        self.send_header('Access-Control-Allow-Headers', 'Content-Type')
        super().end_headers()

    def do_GET(self):
        if self.path.startswith("/uci"):
            self.do_uci()
            return
        super().do_GET()

    def do_uci(self):
        parse_fen = lambda fen: fen.replace("_", "/")
        parsed_url = urlparse(self.path)
        query_params = parse_qs(parsed_url.query)
        fen = parse_fen(query_params["fen"][0])

        with subprocess.Popen(["./target/release/cheng-cmd"], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True) as p:
            p.stdin.write(f"fen {fen}\n")
            p.stdin.write(f"go\n")
            p.stdin.flush()
            bestmove = p.stdout.readline().strip()
            bestmove = bestmove.split(" ")[1]

        self.send_response(200, "OK")
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = { "movement": bestmove }
        self.wfile.write(bytes(json.dumps(response), "utf-8"))

if __name__ == "__main__":
    port = 8000
    server_address = ('', port)
    httpd = HTTPServer(server_address, CORSRequestHandler)
    print(f"Serving on port {port}...")
    httpd.serve_forever()
