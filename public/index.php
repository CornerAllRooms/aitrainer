<?php
declare(strict_types=1);

// Error handling
ini_set('display_errors', 0);
error_reporting(E_ALL);
ini_set('log_errors', 1);
ini_set('error_log', __DIR__.'/../logs/php-errors.log');

// Secure session
session_start([
    'name' => 'SecureSession',
    'cookie_lifetime' => 86400, // 1 day
    'cookie_secure' => true,
    'cookie_httponly' => true,
    'cookie_samesite' => 'Strict',
    'use_strict_mode' => true,
    'use_only_cookies' => 1
]);

// CSRF protection
if (empty($_SESSION['csrf_token'])) {
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
}

// Load environment
require __DIR__.'/../vendor/autoload.php';
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__.'/..');
$dotenv->load();

// MongoDB connection
$client = new MongoDB\Client($_ENV['MONGODB_URI']);
$users = $client->selectCollection('roomie13', 'users1');

// PayFast domain whitelist
$validPayfastDomains = [
    'payfast.co.za',
    'www.payfast.co.za',
    'sandbox.payfast.co.za',
    'payment.payfast.io'
];

// 1. Check if coming from PayFast
if (isset($_SERVER['HTTP_REFERER'])) {
    $referrerHost = parse_url($_SERVER['HTTP_REFERER'], PHP_URL_HOST);
    
    if (in_array($referrerHost, $validPayfastDomains)) {
        // Automatically grant access
        $_SESSION['payfast_verified'] = true;
        
        // Find user by email (assuming PayFast passes it)
        $user = $users->findOne(['email' => $_GET['email'] ?? '']);
        
        if ($user) {
            // Update payment status
            $users->updateOne(
                ['email' => $user['email']],
                ['$set' => [
                    'paymentStatus' => 'active',
                    'lastPaymentDate' => new MongoDB\BSON\UTCDateTime(),
                    'nextBillingDate' => new MongoDB\BSON\UTCDateTime(strtotime('+1 month') * 1000)
                ]]
            );
            
            // Set session
            $_SESSION['user'] = [
                'email' => $user['email'],
                'firstName' => $user['firstName'] ?? '',
                'paymentStatus' => 'active'
            ];
            
            // Redirect to dashboard
            header('Location: /index.jsx');
            exit;
        }
    }
}

// 2. Check existing session
if (isset($_SESSION['user']['email'])) {
    $user = $users->findOne(['email' => $_SESSION['user']['email']]);
    
    if ($user && ($user['paymentStatus'] === 'active' || $_SESSION['payfast_verified'])) {
        header('Location: /index.jsx');
        exit;
    }
}

// 3. Handle form submissions
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    // Validate CSRF token
    if (!isset($_POST['csrf_token'], $_SESSION['csrf_token']) || 
        !hash_equals($_SESSION['csrf_token'], $_POST['csrf_token'])) {
        header('Location: /?error=invalid_csrf');
        exit;
    }

    try {
        if (isset($_POST['signUp'])) {
            // Registration logic
            $existingUser = $users->findOne(['email' => $_POST['email']]);
            
            if ($existingUser) {
                header('Location: /?error=email_exists');
                exit;
            }

            $result = $users->insertOne([
                'firstName' => htmlspecialchars($_POST['fName']),
                'lastName' => htmlspecialchars($_POST['lName']),
                'email' => $_POST['email'],
                'password' => password_hash($_POST['password'], PASSWORD_BCRYPT),
                'createdAt' => new MongoDB\BSON\UTCDateTime(),
                'timezone' => 'Africa/Johannesburg',
                'status' => 'active',
                'role' => 'user',
                'paymentStatus' => 'inactive',
                'lastPaymentDate' => null,
                'nextBillingDate' => null,
                'subscriptionId' => null
            ]);

            if ($result->getInsertedCount() === 1) {
                $_SESSION['user'] = [
                    'email' => $_POST['email'],
                    'firstName' => $_POST['fName']
                ];
                header('Location: /subscribe.php');
                exit;
            }

        } elseif (isset($_POST['signIn'])) {
            // Login logic
            $user = $users->findOne(['email' => $_POST['email']]);
            
            if (!$user || !password_verify($_POST['password'], $user['password'])) {
                header('Location: /?error=invalid_credentials');
                exit;
            }

            if ($user['status'] === 'blocked') {
                header('Location: /?error=account_blocked');
                exit;
            }

            // Update last login
            $users->updateOne(
                ['_id' => $user['_id']],
                ['$set' => ['lastLogin' => new MongoDB\BSON\UTCDateTime()]]
            );

            // Set session
            $_SESSION['user'] = [
                'email' => $user['email'],
                'firstName' => $user['firstName'],
                'paymentStatus' => $user['paymentStatus']
            ];

            // Redirect based on payment status
            if ($user['paymentStatus'] === 'active') {
                header('Location: /index.jsx');
            } else {
                header('Location: /subscribe.php');
            }
            exit;
        }
    } catch (Exception $e) {
        error_log('Auth error: '.$e->getMessage());
        header('Location: /?error=system_error');
        exit;
    }
}
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Login/Register</title>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css">
    <style>
        /* Your existing CSS styles */
        body { font-family: 'Inter', sans-serif; }
        .container { max-width: 400px; margin: 50px auto; padding: 20px; }
        .error-message { color: red; margin-bottom: 15px; }
        .input-group { margin-bottom: 15px; position: relative; }
        .input-group i { position: absolute; left: 10px; top: 10px; }
        input[type="text"], 
        input[type="email"], 
        input[type="password"] {
            width: 100%;
            padding: 10px 10px 10px 35px;
            border: 1px solid #ddd;
            border-radius: 4px;
        }
        .btn { width: 100%; padding: 10px; background: #4CAF50; color: white; border: none; border-radius: 4px; }
        .password-toggle { position: absolute; right: 10px; top: 10px; cursor: pointer; }
    </style>
</head>
<body>
    <?php if (!empty($_GET['error'])): ?>
        <div class="error-message">Error: <?= htmlspecialchars($_GET['error']) ?></div>
    <?php endif; ?>

    <div class="container" id="signIn">
        <h1>Sign In</h1>
        <form method="POST">
            <input type="hidden" name="signIn" value="1">
            <input type="hidden" name="csrf_token" value="<?= htmlspecialchars($_SESSION['csrf_token']) ?>">
            
            <div class="input-group">
                <i class="fas fa-envelope"></i>
                <input type="email" name="email" placeholder="Email" required>
            </div>
            
            <div class="input-group">
                <i class="fas fa-lock"></i>
                <input type="password" name="password" id="password" placeholder="Password" required>
                <span class="password-toggle" onclick="togglePassword('password')">
                    <i class="fas fa-eye"></i>
                </span>
            </div>
            
            <button type="submit" class="btn">Sign In</button>
        </form>
        <p>Don't have an account? <a href="#" onclick="showSignUp()">Sign up</a></p>
    </div>

    <div class="container" id="signUp" style="display:none;">
        <h1>Register</h1>
        <form method="POST">
            <input type="hidden" name="signUp" value="1">
            <input type="hidden" name="csrf_token" value="<?= htmlspecialchars($_SESSION['csrf_token']) ?>">
            
            <div class="input-group">
                <i class="fas fa-user"></i>
                <input type="text" name="fName" placeholder="First Name" required pattern="[A-Za-z]{2,}">
            </div>
            
            <div class="input-group">
                <i class="fas fa-user"></i>
                <input type="text" name="lName" placeholder="Last Name" required pattern="[A-Za-z]{2,}">
            </div>
            
            <div class="input-group">
                <i class="fas fa-envelope"></i>
                <input type="email" name="email" placeholder="Email" required>
            </div>
            
            <div class="input-group">
                <i class="fas fa-lock"></i>
                <input type="password" name="password" id="regPassword" placeholder="Password" required
                       minlength="8" pattern="(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}">
                <span class="password-toggle" onclick="togglePassword('regPassword')">
                    <i class="fas fa-eye"></i>
                </span>
            </div>
            
            <button type="submit" class="btn">Sign Up</button>
        </form>
        <p>Already have an account? <a href="#" onclick="showSignIn()">Sign in</a></p>
    </div>

    <script>
        function togglePassword(id) {
            const input = document.getElementById(id);
            const icon = input.nextElementSibling.querySelector('i');
            if (input.type === 'password') {
                input.type = 'text';
                icon.classList.replace('fa-eye', 'fa-eye-slash');
            } else {
                input.type = 'password';
                icon.classList.replace('fa-eye-slash', 'fa-eye');
            }
        }

        function showSignUp() {
            document.getElementById('signIn').style.display = 'none';
            document.getElementById('signUp').style.display = 'block';
        }

        function showSignIn() {
            document.getElementById('signUp').style.display = 'none';
            document.getElementById('signIn').style.display = 'block';
        }

        // Show signup form if URL has ?show=signup
        if (window.location.search.includes('show=signup')) {
            showSignUp();
        }
    </script>
</body>
</html>