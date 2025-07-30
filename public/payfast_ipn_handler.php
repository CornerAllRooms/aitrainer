<?php
declare(strict_types=1);

// Security headers
header('X-Frame-Options: DENY');
header('X-Content-Type-Options: nosniff');
header('Referrer-Policy: no-referrer');

// Load environment and dependencies
require __DIR__.'/../config/database.php';
require __DIR__.'/../vendor/autoload.php';

use MongoDB\Client as MongoClient;
use MongoDB\BSON\UTCDateTime;

// Initialize MongoDB
try {
    $client = new MongoClient($_ENV['MONGODB_URI'], [
        'tls' => true,
        'retryWrites' => true,
        'w' => 'majority'
    ]);
    $users = $client->selectCollection('roomie13', 'users1');
    $transactions = $client->selectCollection('roomie13', 'payfast_transactions');
} catch (Exception $e) {
    error_log('MongoDB connection failed: '.$e->getMessage());
    http_response_code(500);
    exit;
}

// Validate PayFast IPN
function validateIPN(array $data): bool {
    // 1. Verify PayFast signature
    $pfParamString = '';
    foreach ($data as $key => $val) {
        if ($key !== 'signature') {
            $pfParamString .= $key.'='.urlencode(trim($val)).'&';
        }
    }
    $pfParamString = substr($pfParamString, 0, -1);
    $calculatedSignature = md5($pfParamString.'&passphrase='.urlencode($_ENV['PAYFAST_PASSPHRASE']));

    if ($calculatedSignature !== ($data['signature'] ?? '')) {
        error_log('Signature mismatch: '.$calculatedSignature.' vs '.($data['signature'] ?? ''));
        return false;
    }

    // 2. Validate payment status
    if (($data['payment_status'] ?? '') !== 'COMPLETE') {
        error_log('Payment not complete: '.($data['payment_status'] ?? ''));
        return false;
    }

    // 3. Verify merchant credentials
    if (($data['merchant_id'] ?? '') !== $_ENV['PAYFAST_MERCHANT_ID'] || 
        ($data['merchant_key'] ?? '') !== $_ENV['PAYFAST_MERCHANT_KEY']) {
        error_log('Merchant credential mismatch');
        return false;
    }

    return true;
}

// Main IPN processing
try {
    $rawData = file_get_contents('php://input');
    $postData = json_decode($rawData, true) ?: $_POST;

    // Debug logging (remove in production)
    file_put_contents('ipn.log', date('[Y-m-d H:i:s]').print_r($postData, true), FILE_APPEND);

    if (!validateIPN($postData)) {
        throw new Exception('Invalid IPN request');
    }

    // Extract user email (passed during payment initiation)
    $userEmail = $postData['custom_str1'] ?? null;
    if (!$userEmail) {
        throw new Exception('Missing user identifier');
    }

    // Start transaction
    $session = $client->startSession();
    $session->startTransaction();

$this->collection->updateOne(
    ['email' => $userEmail],
    [
        '$set' => [
            'paymentStatus' => 'active',
            'lastPaymentDate' => new UTCDateTime(),
            'nextBillingDate' => new UTCDateTime(strtotime('+1 month') * 1000),
            'subscriptionId' => $postData['token'],
            'plan' => 'monthly'
        ]
    ]
);
    
    try {
        // 1. Update user subscription status
        $updateResult = $users->updateOne(
            ['email' => $userEmail],
            ['$set' => [
                'paymentStatus' => 'active',
                'lastPaymentDate' => new UTCDateTime(),
                'subscriptionId' => $postData['token'] ?? null,
                'subscriptionType' => $postData['subscription_type'] ?? 'once-off'
            ]],
            ['session' => $session]
        );

        // 2. Record transaction
        $transactions->insertOne([
            'pf_payment_id' => $postData['pf_payment_id'],
            'user_email' => $userEmail,
            'amount_gross' => (float) $postData['amount_gross'],
            'payment_date' => new UTCDateTime(),
            'payment_status' => $postData['payment_status'],
            'item_name' => $postData['item_name'] ?? '',
            'raw_data' => $postData
        ], ['session' => $session]);

        $session->commitTransaction();
    } catch (Exception $e) {
        $session->abortTransaction();
        throw $e;
    }

    // 3. Optional: Trigger welcome email or other post-payment actions
    if ($updateResult->getModifiedCount() > 0) {
        // Example: Notify admin or user
        file_put_contents('payment_success.log', 
            date('[Y-m-d H:i:s]')." Payment processed for $userEmail\n", 
            FILE_APPEND);
    }

$this->collection->updateMany(
    [
        'nextBillingDate' => ['$lt' => new UTCDateTime()],
        'paymentStatus' => 'active'
    ],
    ['$set' => ['paymentStatus' => 'past_due']]
);

    http_response_code(200);
    echo 'OK';
} catch (Exception $e) {
    error_log('IPN processing failed: '.$e->getMessage());
    http_response_code(400);
    echo 'ERROR';
}