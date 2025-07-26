<?php
// Strict error reporting
declare(strict_types=1);
ini_set('display_errors', 0);
error_reporting(E_ALL);
ini_set('log_errors', 1);
ini_set('error_log', __DIR__ . '/../logs/php-errors.log');

// Secure session initialization
session_start([
    'name' => 'SecureSession',
    'cookie_lifetime' => 86400,
    'cookie_secure' => true,
    'cookie_httponly' => true,
    'cookie_samesite' => 'Strict',
    'use_strict_mode' => true,
    'use_only_cookies' => 1
]);

// CSRF token generation
if (empty($_SESSION['csrf_token'])) {
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
}

// Environment variables
require __DIR__.'/../vendor/autoload.php';
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__.'/..');
$dotenv->load();

// Generate secure app key
$app_key = 'base64:' . base64_encode(random_bytes(32));
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Register & Login</title>
    <link rel="icon" href="https://cornerroom.co.za/logo.png" type="image/x-icon">
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css">
    <link rel="stylesheet" href="/assets/style.css">
</head>
<body>
    <?php if (!empty($_GET['error'])): ?>
        <div class="error-message"><?= htmlspecialchars($_GET['error'], ENT_QUOTES) ?></div>
    <?php endif; ?>

    <!-- Sign Up Container -->
    <div class="container" id="signup" style="display:none;">
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
        <p class="or">or</p>
        <div class="icons">
        </div>
        <div class="links">
            <p>Already Have an Account?</p>
            <button id="signInButton">Sign In</button>
            <p2>Whatsapp us to<a href="https://wa.me/p/9863075963803861/27615121021" target="_blank">cancel</a> your subscription with your full name and email</p2>
        </div>
    </div>

    <!-- Sign In Container -->
    <div class="container" id="signIn">
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
                <a href="https://lobby.cornerroom.co.za/reset-password.php">Recover Password</a>
            </p>
            <input type="submit" class="btn" value="Sign In">
        </form>
        <p class="or">or</p>
        <div class="icons">
        </div>
        <div class="links">
            <p>Don't Have an Account Yet?</p>
            <button id="signUpButton">Sign Up</button>
        </div>
    </div>

    <script>
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

        document.getElementById('signUpButton').addEventListener('click', () => {
            document.getElementById('signIn').style.display = 'none';
            document.getElementById('signup').style.display = 'block';
        });

        document.getElementById('signInButton').addEventListener('click', () => {
            document.getElementById('signup').style.display = 'none';
            document.getElementById('signIn').style.display = 'block';
        });

        function handleGoogleSignIn(response) {
            fetch('/google-auth.php', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'X-CSRF-Token': '<?= $_SESSION['csrf_token'] ?>'
                },
                body: JSON.stringify({ credential: response.credential })
            })
            .then(response => response.json())
            .then(data => {
                if (data.success) {
                    window.location.href = '/index.jsx';
                } else {
                    alert('Error: ' + data.error);
                }
            });
        }
    </script>
    <script src="https://accounts.google.com/gsi/client" async defer></script>
</body>
</html>