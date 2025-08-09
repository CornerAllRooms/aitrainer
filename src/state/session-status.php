<?php
declare(strict_types=1);

// Secure session configuration (must match register.php)
session_start([
    'name' => 'SecureSession',
    'cookie_lifetime' => 86400,
    'cookie_secure' => true,
    'cookie_httponly' => true,
    'cookie_samesite' => 'lax',
    'use_strict_mode' => true
]);

header('Content-Type: application/json');

try {
    require __DIR__.'/../vendor/autoload.php';
    $dotenv = Dotenv\Dotenv::createImmutable(__DIR__.'/..');
    $dotenv->load();

    // Default unauthenticated response
    $response = [
        'authenticated' => false,
        'user' => null,
        'csrf_token' => $_SESSION['csrf_token'] ?? null
    ];

    // Check if user is authenticated
    if (isset($_SESSION['user']['email']) && ($_SESSION['user']['logged_in'] ?? false)) {
        // Connect to MongoDB to get full user data
        $client = new MongoDB\Client(
            $_ENV['MONGODB_URI'],
            [
                'tls' => true,
                'retryWrites' => true,
                'w' => 'majority'
            ]
        );
        
        $collection = $client->selectCollection('roomie13', 'users1');
        $user = $collection->findOne(['email' => $_SESSION['user']['email']]);

        if ($user) {
            // Format user data for client
            $response = [
                'authenticated' => true,
                'user' => [
                    'id' => (string)$user['_id'],
                    'email' => $user['email'],
                    'firstName' => $user['firstName'] ?? '',
                    'lastName' => $user['lastName'] ?? '',
                    'role' => $user['role'] ?? 'user',
                    'status' => $user['status'] ?? 'active',
                    'timezone' => $user['timezone'] ?? 'Africa/Johannesburg',
                    'paymentStatus' => $user['paymentStatus'] ?? 'inactive'
                ],
                'csrf_token' => $_SESSION['csrf_token'] ?? null
            ];

            // Check if account is blocked
            if (($user['status'] ?? 'active') === 'blocked') {
                $response['authenticated'] = false;
                $response['blocked'] = true;
                session_destroy();
            }
        } else {
            // User not found in database - destroy session
            session_destroy();
        }
    }

    // Security headers
    header('X-Content-Type-Options: nosniff');
    header('X-Frame-Options: DENY');
    header('X-XSS-Protection: 1; mode=block');

    echo json_encode($response);

} catch (Exception $e) {
    // Error response
    http_response_code(500);
    echo json_encode([
        'authenticated' => false,
        'error' => 'Session check failed',
        'csrf_token' => null
    ]);
    error_log('session-status.php error: ' . $e->getMessage());
}
?>