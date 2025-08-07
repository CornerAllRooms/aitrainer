
<?php
require 'vendor/autoload.php';

use MongoDB\Client;

// Load environment variables
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__);
$dotenv->load();
$dotenv->required('MONGODB_URI');

try {
    $client = new MongoDB\Client(
        $_ENV['MONGODB_URI'], 
        [
            'tls' => true,
            'authMechanism' => 'SCRAM-SHA-256',
            'connectTimeoutMS' => 3000,
            'socketTimeoutMS' => 30000
        ]
    );
    
    // Test connection
    $client->selectDatabase('admin')->command(['ping' => 1]);
    
    return $client; // Or assign to variable

} catch (Exception $e) {
    error_log("MongoDB connection failed: " . $e->getMessage());
    throw $e; // Or handle gracefully
}
?>
