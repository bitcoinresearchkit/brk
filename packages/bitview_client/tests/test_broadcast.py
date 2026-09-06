"""Local-only broadcast transport regression; no Bitcoin node is contacted."""
import threading
import unittest
from http.server import BaseHTTPRequestHandler, HTTPServer

from bitview_client import BitviewClient, BitviewError


class BroadcastTest(unittest.TestCase):
    def test_plain_text_rejection_and_no_replay(self):
        calls = []

        class Handler(BaseHTTPRequestHandler):
            def do_POST(self):
                body = self.rfile.read(int(self.headers["Content-Length"]))
                calls.append((self.command, self.path, body))
                if len(calls) == 3:
                    self.close_connection = True
                    return
                data = b'{"error":"rejected"}' if len(calls) == 2 else b'a' * 64
                self.send_response(400 if len(calls) == 2 else 200)
                self.send_header("Content-Type", "text/plain")
                self.send_header("Content-Length", str(len(data)))
                self.end_headers()
                self.wfile.write(data)

            def log_message(self, *_args):
                pass

        server = HTTPServer(("127.0.0.1", 0), Handler)
        worker = threading.Thread(target=server.serve_forever)
        worker.start()
        try:
            with BitviewClient(f"http://127.0.0.1:{server.server_port}", timeout=2) as client:
                self.assertEqual(client.post_tx("aa"), "a" * 64)
                with self.assertRaises(BitviewError):
                    client.post_tx("bb")
                with self.assertRaises(BitviewError):
                    client.post_tx("cc")
            self.assertEqual(calls, [("POST", "/api/tx", body) for body in [b"aa", b"bb", b"cc"]])
        finally:
            server.shutdown()
            worker.join()
            server.server_close()


if __name__ == "__main__":
    unittest.main()
