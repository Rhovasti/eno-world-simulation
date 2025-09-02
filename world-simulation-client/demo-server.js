#!/usr/bin/env node

const http = require('http');
const fs = require('fs');
const path = require('path');

const PORT = 3002;

const mimeTypes = {
  '.html': 'text/html',
  '.js': 'application/javascript',
  '.css': 'text/css',
  '.json': 'application/json',
  '.png': 'image/png',
  '.jpg': 'image/jpg',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.wav': 'audio/wav',
  '.mp4': 'video/mp4',
  '.woff': 'application/font-woff',
  '.ttf': 'application/font-ttf',
  '.eot': 'application/vnd.ms-fontobject',
  '.otf': 'application/font-otf',
  '.wasm': 'application/wasm'
};

const server = http.createServer((req, res) => {
  console.log(`${req.method} ${req.url}`);

  // Remove query parameters and handle root
  let pathname = req.url.split('?')[0];
  if (pathname === '/') {
    pathname = '/demo/simple-demo.html';
  }

  const filePath = path.join(__dirname, pathname);
  const extname = String(path.extname(filePath)).toLowerCase();
  const mimeType = mimeTypes[extname] || 'application/octet-stream';

  fs.readFile(filePath, (error, content) => {
    if (error) {
      if (error.code === 'ENOENT') {
        // File not found
        res.writeHead(404, { 'Content-Type': 'text/html' });
        res.end('<h1>404 - File Not Found</h1><p>Available paths:</p><ul><li><a href="/demo/simple-demo.html">Simple Demo</a></li></ul>', 'utf-8');
      } else {
        // Server error
        res.writeHead(500);
        res.end(`Server Error: ${error.code}`, 'utf-8');
      }
    } else {
      // Success
      res.writeHead(200, { 
        'Content-Type': mimeType,
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
        'Access-Control-Allow-Headers': 'Content-Type, Authorization'
      });
      res.end(content, 'utf-8');
    }
  });
});

server.listen(PORT, () => {
  console.log(`🌍 World Simulation Client Demo Server`);
  console.log(`📡 Server running at http://localhost:${PORT}`);
  console.log(`🎮 Demo available at http://localhost:${PORT}/demo/simple-demo.html`);
  console.log(`📚 Testing Guide:`);
  console.log(`   1. Open the demo URL in your browser`);
  console.log(`   2. Click "Test Connection" to begin`);
  console.log(`   3. Use other buttons to test features`);
  console.log(`   4. For full testing: npm run example:quick`);
  console.log(`\n🔧 To stop server: Ctrl+C`);
});