<?php
declare(strict_types=1);
error_reporting(E_ALL);
ini_set('display_errors', '1'); // Temporarily enable for debugging
ini_set('log_errors', '1');

// Security headers
header("X-Frame-Options: DENY");
header("X-Content-Type-Options: nosniff");
header("Referrer-Policy: strict-origin-when-cross-origin");
header("Content-Security-Policy: default-src 'self'; script-src 'self' 'unsafe-inline'");
header("Strict-Transport-Security: max-age=31536000; includeSubDomains");

// Load dependencies
require __DIR__.'/../config/database.php';
require __DIR__.'/../config/settings.php';
require __DIR__.'/../config/MailjetMailer.php';
require __DIR__.'/../vendor/autoload.php';

// Initialize environment
$dotenv = Dotenv\Dotenv::createImmutable(__DIR__.'/..');
$dotenv->safeLoad();

// Session configuration
session_start([
    'cookie_httponly' => true,
    'cookie_secure' => true,
    'cookie_samesite' => 'Strict'
]);

if (!isset($_SESSION['reset_attempts'])) {
    $_SESSION['reset_attempts'] = 0;
}

/**
 * Render the password reset form with CSRF protection
 */
function renderForm(?string $token = null, ?string $message = null, ?string $error = null): void {
    $csrfToken = bin2hex(random_bytes(32));
    $_SESSION['csrf_token'] = $csrfToken;
    ?>
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Reset Password</title>
        <link rel="icon" href="https://cornerroom.co.za/original.png" type="image/x-icon">
        <link rel="stylesheet" href="/assets/reset.css">
    </head>
    <body>
        <div class="container">
            <h1 class="form-title">Reset Password</h1>

            <?php if ($message): ?>
                <div class="message success"><?= htmlspecialchars($message, ENT_QUOTES, 'UTF-8') ?></div>
            <?php endif; ?>

            <?php if ($error): ?>
                <div class="message error"><?= htmlspecialchars($error, ENT_QUOTES, 'UTF-8') ?></div>
            <?php endif; ?>

            <?php if ($token): ?>
                <form method="POST" action="/reset-handler.php" autocomplete="off" novalidate>
                    <input type="hidden" name="token" value="<?= htmlspecialchars($token, ENT_QUOTES, 'UTF-8') ?>">
                    <input type="hidden" name="csrf_token" value="<?= $csrfToken ?>">

                    <div class="input-group">
                        <input type="password" id="password" name="password" required minlength="12"
                               pattern="(?=.*\d)(?=.*[a-z])(?=.*[A-Z]).{12,}"
                               title="Must contain at least 12 characters, including uppercase, lowercase and numbers"
                               placeholder="New Password" aria-label="New Password">
                    </div>
                    <div class="input-group">
                        <input type="password" id="confirm_password" name="confirm_password" required minlength="12"
                               placeholder="Confirm Password" aria-label="Confirm Password">
                    </div>
                    <button type="submit" class="btn" aria-label="Reset password">Reset Password</button>
                </form>
            <?php else: ?>
                <form method="POST" novalidate>
                    <input type="hidden" name="csrf_token" value="<?= $csrfToken ?>">
                    <div class="input-group">
                        <input type="email" name="email" required placeholder="Your Email" aria-label="Your Email">
                    </div>
                    <button type="submit" class="btn" aria-label="Send reset link">Send Reset Link</button>
                </form>
                <div class="recover">
                    <a href="/index.php">Remember your password? Log in</a>
                </div>
            <?php endif; ?>
        </div>

        <script>
            document.addEventListener('DOMContentLoaded', function() {
                const forms = document.querySelectorAll('form');
                forms.forEach(form => {
                    form.addEventListener('submit', function(e) {
                        const password = form.querySelector('#password')?.value;
                        const confirm = form.querySelector('#confirm_password')?.value;

                        if (password && confirm && password !== confirm) {
                            e.preventDefault();
                            alert('Passwords do not match!');
                            return false;
                        }
                        return true;
                    });
                });
            });
        </script>
    </body>
    </html>
    <?php
}

// Main processing
try {
    // Verify Mailjet configuration
    if (!file_exists(__DIR__.'/../config/MailjetMailer.php')) {
        throw new Exception("Mailjet configuration missing");
    }

    // Handle POST requests (password reset requests)
    if ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['email'])) {
        // Rate limiting check
        if ($_SESSION['reset_attempts'] >= 5) {
            renderForm(null, null, "Too many attempts. Please try again later.");
            exit;
        }

        // CSRF validation
        if (!isset($_POST['csrf_token'])) {
            throw new Exception("CSRF token missing");
        }

        if (!hash_equals($_SESSION['csrf_token'] ?? '', $_POST['csrf_token'] ?? '')) {
            throw new Exception("CSRF token mismatch");
        }

        $email = filter_var($_POST['email'], FILTER_SANITIZE_EMAIL);
        if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
            throw new Exception("Invalid email format");
        }

        $manager = new MongoDB\Driver\Manager($config['mongo_uri']);

        // Generate token
        $token = bin2hex(random_bytes(32));
        $expiresAt = new MongoDB\BSON\UTCDateTime((time() + 3600) * 1000); // 1 hour expiry

        // Check if email exists
        $query = new MongoDB\Driver\Query(['email' => $email]);
        $users = $manager->executeQuery('roomie13.users', $query);
        $user = current($users->toArray());

        if ($user) {
            // Store token only if user exists
            $bulk = new MongoDB\Driver\BulkWrite();
            $bulk->update(
                ['email' => $email],
                ['$set' => [
                    'resetToken' => password_hash($token, PASSWORD_DEFAULT),
                    'resetExpires' => $expiresAt
                ]],
                ['multi' => false]
            );
            $manager->executeBulkWrite('roomie13.users', $bulk);

            // Initialize Mailjet
            $mailerConfig = [
                'api_key' => $_ENV['MJ_APIKEY_PUBLIC'] ?? $config['mailjet']['api_key'] ?? '',
                'secret_key' => $_ENV['MJ_APIKEY_PRIVATE'] ?? $config['mailjet']['secret_key'] ?? '',
                'sender_email' => $_ENV['MJ_FROM_EMAIL'] ?? 'no-reply@cornerroom.co.za',
                'sender_name' => $_ENV['MJ_FROM_NAME'] ?? 'Corner Room Lobby'
            ];

            $mailer = new MailjetMailer($mailerConfig);

            // Create both tracking and direct links
            $trackingLink = "https://" . $_SERVER['HTTP_HOST'] . "/reset-password.php?token=" . urlencode($token);
            $directLink = "https://" . $_SERVER['HTTP_HOST'] . "/reset-password.php?token=" . $token;

            // Send with both link types
            if (!$mailer->sendPasswordReset($email, $trackingLink, $directLink)) {
                throw new Exception("Failed to send reset email");
            }

            $_SESSION['reset_attempts']++;
        }

        // Always show success message (security measure)
        renderForm(null, "If this email exists, we've sent a reset link");
        exit;
    }

    // Handle GET requests with token
    if (isset($_GET['token'])) {
        $rawToken = $_GET['token'];

        // Handle Mailjet's encoded URL case
        if (strpos($rawToken, 'http') === 0) {
            $decoded = urldecode($rawToken);
            $queryString = parse_url($decoded, PHP_URL_QUERY);
            parse_str($queryString ?? '', $params);
            $rawToken = $params['token'] ?? '';
        }

        if (!preg_match('/^[a-f0-9]{64}$/', $rawToken)) {
            throw new Exception("Invalid token format");
        }

        $manager = new MongoDB\Driver\Manager($config['mongo_uri']);
        $query = new MongoDB\Driver\Query([
            'resetExpires' => ['$gt' => new MongoDB\BSON\UTCDateTime()]
        ]);

        $users = $manager->executeQuery('roomie13.users', $query);
        $validUser = null;

        foreach ($users as $user) {
            if (isset($user->resetToken) && password_verify($rawToken, $user->resetToken)) {
                $validUser = $user;
                break;
            }
        }

        renderForm($validUser ? $rawToken : null, null, $validUser ? null : "Invalid or expired token");
        exit;
    }

    // Default case (show empty form)
    renderForm();

} catch (Exception $e) {
    error_log("[Password Reset Error] " . $e->getMessage());
    renderForm(null, null, "An error occurred. Please try again.");
    exit;
}