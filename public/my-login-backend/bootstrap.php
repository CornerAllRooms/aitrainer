<?php
declare(strict_types=1);

// Security headers
header("X-Frame-Options: DENY");
header("X-Content-Type-Options: nosniff");
header("X-XSS-Protection: 1; mode=block");
header("Referrer-Policy: strict-origin-when-cross-origin");

// Error reporting
if (($_ENV['APP_ENV'] ?? 'production') === 'development') {
    error_reporting(E_ALL);
    ini_set('display_errors', '1');
} else {
    error_reporting(0);
    ini_set('display_errors', '0');
}

// Timezone
date_default_timezone_set('UTC');

// Autoloader
require __DIR__ . '/vendor/autoload.php';

// Environment setup
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();
$dotenv->required([
    'APP_ENV',
    'MONGODB_URI',
    'GOOGLE_CLIENT_ID',
    'SESSION_SECRET',
    'SESSION_NAME',
    'SESSION_LIFETIME'
])->notEmpty();

// Database connection
$mongoClient = new MongoDB\Client(
    $_ENV['MONGODB_URI'],
    [
        'connectTimeoutMS' => 3000,
        'socketTimeoutMS' => 5000,
        'serverSelectionTimeoutMS' => 5000,
        'tls' => true
    ],
    [
        'typeMap' => [
            'root' => 'array',
            'document' => 'array',
            'array' => 'array'
        ]
    ]
);

// Session configuration
session_name($_ENV['SESSION_NAME']);
session_set_cookie_params([
    'lifetime' => (int)$_ENV['SESSION_LIFETIME'],
    'path' => '/',
    'domain' => $_SERVER['HTTP_HOST'],
    'secure' => ($_ENV['APP_ENV'] === 'production'),
    'httponly' => true,
    'samesite' => 'Strict'
]);
session_start();

// Error handlers
set_error_handler(function($errno, $errstr, $errfile, $errline) {
    error_log("PHP Error [$errno] $errstr in $errfile on line $errline");
    if ($_ENV['APP_ENV'] === 'production') {
        http_response_code(500);
        exit('An error occurred');
    }
    return false;
});

set_exception_handler(function(Throwable $e) {
    error_log("Uncaught Exception: " . $e->getMessage());
    http_response_code(500);
    exit('An unexpected error occurred');
});

register_shutdown_function(function() {
    $error = error_get_last();
    if ($error && in_array($error['type'], [E_ERROR, E_PARSE, E_CORE_ERROR, E_COMPILE_ERROR])) {
        error_log("Fatal Error: " . $error['message']);
        http_response_code(500);
        exit('A fatal error occurred');
    }
});