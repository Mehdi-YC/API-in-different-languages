const http = require('http');

const PORT = 8001;

const server = http.createServer((req, res) => {
  if (req.url === '/json') {
    res.writeHead(200, { 'Content-Type': 'application/json' });
    const data = { message: 'Hello, JSON!' };
    res.end(JSON.stringify(data));
  } else {
    res.writeHead(200, { 'Content-Type': 'text/html' });
    res.end('<html><body><h1>Hello, HTML!</h1></body></html>');
  }
});

server.listen(PORT, () => {
  console.log(`Server started at port ${PORT}`);
});

