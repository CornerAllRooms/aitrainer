<?php
error_reporting(E_ALL);
ini_set('display_errors', 1);

// Log errors to a specific file
ini_set('log_errors', 1);
ini_set('error_log', '/var/www/ai.cornerroom.co.za/php_errors.log');

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
    <title>Register & Login</title>
    <link rel="stylesheet" href="/assets/style.css">
    <link rel="icon" href="https://github.com/CornerAllRooms/home/raw/main/assets/logo.png" type="image/png">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css">
<script src="script.js"></script>
 <div id="root"></div>
        <script type="module" src="/dist/assets/index.js"></script>
    <style>
        /* NUCLEAR FORM VISIBILITY CONTROL */
        #signIn:not(.active-form),
        #signup:not(.active-form) {
            display: none !important;
            height: 0 !important;
            overflow: hidden !important;
        }

        #signIn.active-form,
        #signup.active-form {
            display: block !important;
            height: auto !important;
            animation: fadeIn 0.3s ease;
        }

        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }

        /* Your existing error message styling */
        .error-message {
            background: #ffebee;
            color: #d32f2f;
            padding: 12px 20px;
            border-radius: 4px;
            margin: 0 auto 20px;
            max-width: 450px;
            border-left: 4px solid #d32f2f;
        }
    </style>
</head>
<body>
    <?php if (!empty($_GET['error'])): ?>
        <div class="error-message"><?= htmlspecialchars($_GET['error'], ENT_QUOTES) ?></div>
    <?php endif; ?>

    <!-- Sign In Container -->
    <div class="container active-form" id="signIn">
        <h1 class="form-title">Sign In</h1>
        <form method="post" action="/register.php">
            <input type="hidden" name="signIn" value="1">
            <input type="hidden" name="csrf_token" value="<?= htmlspecialchars($_SESSION['csrf_token'], ENT_QUOTES) ?>">

            <div class="input-group">
                <i class="fas fa-envelope"></i>
                <input type="email" name="email" id="login-email" placeholder="Email" required>
            </div>
            <div class="input-group">
                <i class="fas fa-lock"></i>
                <input type="password" name="password" id="signin-password" placeholder="Password" required>
                <span class="password-toggle" onclick="togglePassword('signin-password')">
                    <i class="fas fa-eye"></i>
                </span>
            </div>
            <p class="recover">
                <a href="/reset-password.php">Recover Password</a>
            </p>
            <input type="submit" class="btn" value="Sign In">
        </form>
        <div class="links">
            <p>Don't Have an Account Yet?</p>
            <button id="signUpButton">Sign Up</button>
        </div>
    </div>

    <!-- Sign Up Container -->
    <div class="container" id="signup">
        <h1 class="form-title">Register</h1>
        <form method="post" action="/register.php">
            <input type="hidden" name="signUp" value="1">
            <input type="hidden" name="csrf_token" value="<?= htmlspecialchars($_SESSION['csrf_token'], ENT_QUOTES) ?>">

            <div class="input-group">
                <i class="fas fa-user"></i>
                <input type="text" name="fName" id="fName" placeholder="First Name" required
                       pattern="[A-Za-z]{2,}" title="At least 2 letters">
            </div>
            <div class="input-group">
                <i class="fas fa-user"></i>
                <input type="text" name="lName" id="lName" placeholder="Last Name" required
                       pattern="[A-Za-z]{2,}" title="At least 2 letters">
            </div>
            <div class="input-group">
                <i class="fas fa-envelope"></i>
                <input type="email" name="email" id="email" placeholder="Email" required>
            </div>
            <div class="input-group">
                <i class="fas fa-lock"></i>
                <input type="password" name="password" id="signup-password" placeholder="Password" required
                       minlength="8" pattern="(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{8,}"
                       title="8+ chars with uppercase, lowercase, and number">
                <span class="password-toggle" onclick="togglePassword('signup-password')">
                    <i class="fas fa-eye"></i>
                </span>
            </div>
            <input type="submit" class="btn" value="Sign Up">
        </form>
        <div class="links">
            <p>Already Have an Account?</p>
            <button id="signInButton">Sign In</button>
        </div>
    </div>

    <script>
        // Toggle password visibility
        function togglePassword(inputId) {
            const input = document.getElementById(inputId);
            const icon = input.nextElementSibling.querySelector('i');
            if (input.type === 'password') {
                input.type = 'text';
                icon.classList.replace('fa-eye', 'fa-eye-slash');
            } else {
                input.type = 'password';
                icon.classList.replace('fa-eye-slash', 'fa-eye');
            }
        }

        // Form switching logic
        document.addEventListener('DOMContentLoaded', function() {
            const signUpButton = document.getElementById('signUpButton');
            const signInButton = document.getElementById('signInButton');
            const signInForm = document.getElementById('signIn');
            const signUpForm = document.getElementById('signup');

            // Initialize form visibility
            function initForms() {
                // First hide both to override any external CSS
                signInForm.classList.remove('active-form');
                signUpForm.classList.remove('active-form');

                // Then show the correct one
                if(window.location.search.includes('show=signup')) {
                    signUpForm.classList.add('active-form');
                } else {
                    signInForm.classList.add('active-form');
                }
            }

            // Toggle forms with full state management
            function showForm(showForm, hideForm) {
                hideForm.classList.remove('active-form');
                showForm.classList.add('active-form');
                window.history.pushState(null, null,
                    showForm === signUpForm ? '?show=signup' : window.location.pathname);
            }

            // Event listeners
            signUpButton.addEventListener('click', (e) => {
                e.preventDefault();
                showForm(signUpForm, signInForm);
            });

            signInButton.addEventListener('click', (e) => {
                e.preventDefault();
                showForm(signInForm, signUpForm);
            });

            // Handle browser back/forward
            window.addEventListener('popstate', initForms);

            // Initialize on load
            initForms();
        });
    </script>
</body>
</html>
