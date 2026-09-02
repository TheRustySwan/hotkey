import http.server
import socketserver
import os
import sys
import webbrowser

HOST = "127.0.0.1"
PORT = 8080
ROOT_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DIRECTORY = os.path.join(ROOT_DIR, "dist")

class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIRECTORY, **kwargs)

    def end_headers(self):
        # Security & WASM MIME headers
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('X-Content-Type-Options', 'nosniff')
        self.send_header('X-Frame-Options', 'DENY')
        self.send_header('Referrer-Policy', 'no-referrer')
        super().end_headers()

if __name__ == '__main__':
    if not os.path.exists(DIRECTORY):
        print(f"Directory {DIRECTORY} does not exist. Run scripts/build-web.ps1 first!")
        sys.exit(1)
    os.chdir(DIRECTORY)
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer((HOST, PORT), Handler) as httpd:
        url = f"http://{HOST}:{PORT}"
        print(f"Hotkey Web server running at: {url}")
        print(f"Serving files from: {DIRECTORY}")
        print("Press Ctrl+C to stop the server.")
        try:
            webbrowser.open(url)
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nShutting down server.")
            sys.exit(0)
