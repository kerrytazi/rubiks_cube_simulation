#!/usr/bin/env python3
from http.server import HTTPServer, SimpleHTTPRequestHandler
import os, sys

os.chdir(sys.argv[1])

class Handler(SimpleHTTPRequestHandler):
	def end_headers(self):
		self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
		self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
		super().end_headers()

HTTPServer(('', 8000), Handler).serve_forever()
