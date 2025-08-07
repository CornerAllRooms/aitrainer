<?php
session_start();

$allowed_targets = [
    'dashboard' => [
        'path' => 'menu_and_logged_in/log_in.html',
        'verify' => function() {
            return isset($_SESSION['logged_in']);
        }
    ]
];

$target = $_GET['target'] ?? '';
if (isset($allowed_targets[$target]) {
    if ($allowed_targets[$target]['verify']()) {
        header("Location: " . $allowed_targets[$target]['path']);
        exit;
    }
}

// Failed access
header("HTTP/1.1 403 Forbidden");
readfile('fallback_error.png'); // Show error image
?>