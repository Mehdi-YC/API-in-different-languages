<?php
$port = 8006;

if ($_SERVER['REQUEST_URI'] === '/json') {
    header('Content-Type: application/json');
    echo json_encode(['message' => 'Hello, JSON!']);
} else {
    header('Content-Type: text/html');
    echo '<html><body><h1>Hello, HTML PHP!</h1></body></html>';
}
?>