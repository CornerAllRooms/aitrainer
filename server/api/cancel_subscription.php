<?php
session_start();
require_once __DIR__.'/../config/database.php';

// Validate CSRF
if (!isset($_POST['csrf_token']) || $_POST['csrf_token'] !== $_SESSION['csrf_token']) {
    http_response_code(403);
    die('Invalid CSRF token');
}

// Check authentication
if (!isset($_SESSION['user']['email'])) {
    http_response_code(401);
    die('Unauthorized');
}

// Database connection
$client = new MongoDB\Client($_ENV['MONGODB_URI']);
$collection = $client->selectCollection('roomie13', 'users');

// Get user's subscription ID
$user = $collection->findOne([
    'email' => $_SESSION['user']['email'],
    'paymentStatus' => 'active'
]);

if (!$user || !isset($user['subscriptionId'])) {
    http_response_code(400);
    die('No active subscription found');
}

// Initiate PayFast cancellation
$payload = [
    'subscription_id' => $user['subscriptionId'],
    'cancel' => true
];

// Send to PayFast API (using cURL)
$ch = curl_init('https://api.payfast.co.za/subscriptions/' . $user['subscriptionId'] . '/cancel');
curl_setopt_array($ch, [
    CURLOPT_RETURNTRANSFER => true,
    CURLOPT_CUSTOMREQUEST => 'PUT',
    CURLOPT_POSTFIELDS => http_build_query($payload),
    CURLOPT_HTTPHEADER => [
        'Authorization: Bearer ' . $_ENV['PAYFAST_MERCHANT_KEY'],
        'Content-Type: application/x-www-form-urlencoded'
    ]
]);

$response = curl_exec($ch);
$status = curl_getinfo($ch, CURLINFO_HTTP_CODE);
curl_close($ch);

// Handle response
if ($status === 200) {
    // Update database
    $collection->updateOne(
        ['email' => $_SESSION['user']['email']],
        ['$set' => [
            'paymentStatus' => 'cancelled',
            'subscriptionCancelledAt' => new MongoDB\BSON\UTCDateTime()
        ]]
    );
    
    header('Content-Type: application/json');
    echo json_encode(['success' => true]);
} else {
    http_response_code(400);
    echo json_encode(['error' => 'Cancellation failed']);
}