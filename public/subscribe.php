<?php
declare(strict_types=1);

// Secure session and CSRF
session_start([
    'cookie_lifetime' => 86400,
    'cookie_secure' => true,
    'cookie_httponly' => true,
    'cookie_samesite' => 'Lax'
]);

require __DIR__.'/../config/database.php';
require __DIR__.'/../vendor/autoload.php';

use MongoDB\Client as MongoClient;

// Check authentication
if (!isset($_SESSION['user']['email'])) {
    header('Location: /index.php?redirect=/subscribe.php');
    exit;
}

// Database connection
$client = new MongoClient($_ENV['MONGODB_URI']);
$collection = $client->selectCollection('roomie13', 'users1');

// Check if user already has active subscription
$user = $collection->findOne([
    'email' => $_SESSION['user']['email'],
    'paymentStatus' => 'active'
]);

if ($user) {
    header('Location: /profile.php?message=You+already+have+an+active+subscription');
    exit;
}

// Generate CSRF token if not exists
if (!isset($_SESSION['csrf_token'])) {
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
}

// Process form submission
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    if (!isset($_POST['csrf_token']) || !hash_equals($_SESSION['csrf_token'], $_POST['csrf_token'])) {
        die('CSRF validation failed');
    }

    // Prepare PayFast subscription data
    $payfastData = [
        'merchant_id' => $_ENV['PAYFAST_MERCHANT_ID'],
        'merchant_key' => $_ENV['PAYFAST_MERCHANT_KEY'],
        'return_url' => 'https://ai.cornerroom.co.za/subscribe/success',
        'cancel_url' => 'https://ai.cornerroom.co.za/subscribe/cancel',
        'notify_url' => 'https://ai.cornerroom.co.za/payfast_ipn_handler.php',
        'name_first' => $_SESSION['user']['firstName'] ?? '',
        'name_last' => $_SESSION['user']['lastName'] ?? '',
        'email_address' => $_SESSION['user']['email'],
        'm_payment_id' => 'sub_' . bin2hex(random_bytes(8)),
        'amount' => '49.99',
        'item_name' => 'AI Trainer Monthly Subscription',
        'subscription_type' => 1,
        'recurring_amount' => '49.99',
        'frequency' => 3, // Monthly
        'cycles' => 0, // Infinite
        'custom_str1' => $_SESSION['user']['email'] // Identify user
    ];

    // Generate signature
    $pfOutput = '';
    foreach ($payfastData as $key => $val) {
        if (!empty($val)) {
            $pfOutput .= $key . '=' . urlencode(trim($val)) . '&';
        }
    }
    $pfOutput = substr($pfOutput, 0, -1);
    $payfastData['signature'] = md5($pfOutput . '&passphrase=' . urlencode($_ENV['PAYFAST_PASSPHRASE']));

    // Store pending subscription in database
    $collection->updateOne(
        ['email' => $_SESSION['user']['email']],
        ['$set' => [
            'pendingSubscription' => true,
            'subscriptionAttemptAt' => new MongoDB\BSON\UTCDateTime()
        ]]
    );

    // Redirect to PayFast
    header('Location: https://www.payfast.co.za/eng/process?' . $pfOutput);
    exit;
}
?>

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Subscribe to AI Trainer</title>
    <link rel="stylesheet" href="/assets/subscribe.css">
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap" rel="stylesheet">
</head>
<body>
    <div class="subscribe-container">
        <div class="subscribe-card">
            <h1> AI Trainer Subscription</h1>
            <div class="pricing-card">
                <div class="price">R49.99<span class="period">/month</span></div>
                <ul class="features">
                    <li>✔ Unlimited AI training sessions</li>
                    <li>✔ Priority support</li>
                    <li>✔ Exclusive features</li>
                    <li>✔ Cancel anytime</li>
                </ul>
            </div>

            <form method="POST" class="subscription-form">
                <input type="hidden" name="csrf_token" value="<?= htmlspecialchars($_SESSION['csrf_token']) ?>">
                
                <div class="payment-method">
                    <div class="method-header">
                        <svg viewBox="0 0 24 24" class="method-icon">
                            <path d="M5 6h14a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2z"/>
                            <circle cx="8" cy="12" r="1"/>
                            <circle cx="12" cy="12" r="1"/>
                            <circle cx="16" cy="12" r="1"/>
                        </svg>
                        <span>PayFast Secure Payment</span>
                    </div>
                    <p class="method-description">
                        You'll be redirected to PayFast to complete your subscription securely.
                    </p>
                </div>

                <button type="submit" class="subscribe-button">
                    Subscribe Now
                </button>
            </form>

            <div class="security-notice">
                <svg viewBox="0 0 24 24" class="lock-icon">
                    <path d="M12 2C9.243 2 7 4.243 7 7v3H6a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8a2 2 0 0 0-2-2h-1V7c0-2.757-2.243-5-5-5zM12 4c1.654 0 3 1.346 3 3v3H9V7c0-1.654 1.346-3 3-3z"/>
                </svg>
                <span>256-bit SSL encryption. Your data is always secure.</span>
            </div>
        </div>
    </div>
</body>
</html>