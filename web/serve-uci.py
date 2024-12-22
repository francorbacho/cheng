#!/usr/bin/python3

from http.server import HTTPServer, SimpleHTTPRequestHandler
import json

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
        self.send_response(200, "OK")
        self.send_header('Content-Type', 'application/json')
        self.end_headers()
        response = { "movement": "e2e4" }
        self.wfile.write(bytes(json.dumps(response), "utf-8"))

if __name__ == "__main__":
    port = 8000
    server_address = ('', port)
    httpd = HTTPServer(server_address, CORSRequestHandler)
    print(f"Serving on port {port}...")
    httpd.serve_forever()
