<?php
declare(strict_types=1);
// Secure session
session_start([
    'name' => 'SecureSession',
    'cookie_lifetime' => 86400,
    'cookie_secure' => true,
    'cookie_httponly' => true,
    'cookie_samesite' => 'lax',
    'use_strict_mode' => true
]);
require __DIR__.'/../config/MailjetMailer.php';
require __DIR__.'/../config/settings.php';
require __DIR__.'/../vendor/autoload.php';

use Dotenv\Dotenv;
use MongoDB\Client as MongoClient;
use MongoDB\BSON\UTCDateTime;

// Load environment
$dotenv = Dotenv::createImmutable(__DIR__.'/..');
$dotenv->load();

// Verify CSRF token FIRST
if ($_SERVER['REQUEST_METHOD'] === 'POST') {
    if (!isset($_POST['csrf_token'], $_SESSION['csrf_token']) ||
        !hash_equals($_SESSION['csrf_token'], $_POST['csrf_token'])) {

        error_log('CSRF validation failed. Session: ' . ($_SESSION['csrf_token'] ?? 'NULL') .
                 ' | Posted token: ' . ($_POST['csrf_token'] ?? 'NULL'));

        http_response_code(403);
        header('Location: /index.php?error=CSRF token validation failed');
        exit;
    }

    // Regenerate token after successful validation
    $_SESSION['csrf_token'] = bin2hex(random_bytes(32));
}

class AuthHandler {
    private $collection;

    public function __construct() {
        try {
            $client = new MongoClient(
                $_ENV['MONGODB_URI'],
                [
                    'tls' => true,
                    'retryWrites' => true,
                    'w' => 'majority'
                ]
            );
            $this->collection = $client->selectCollection('roomie13', 'users1');
        } catch (Exception $e) {
            $this->handleError("Database connection failed", 500);
        }
    }

    public function handleRequest(): void {
        if ($_SERVER['REQUEST_METHOD'] !== 'POST') {
            $this->handleError("Invalid request method", 405);
        }

        try {
            if (isset($_POST['signUp'])) {
                $this->handleSignUp();
            } elseif (isset($_POST['signIn'])) {
                $this->handleSignIn();
            } elseif (isset($_POST['getActivity'])) {
                $this->handleGetActivity();
            } elseif (isset($_POST['blockUser'])) {
                $this->handleBlockUser();
            } elseif (isset($_POST['unblockUser'])) {
                $this->handleUnblockUser();
            } else {
                $this->handleError("Invalid action", 400);
            }
        } catch (Exception $e) {
            $this->handleError($e->getMessage(), 500);
        }
    }

    private function handleSignUp(): void {
        $data = $this->validateSignUpData();

        $existingUser = $this->collection->findOne(['email' => $data['email']]);
        if ($existingUser) {
            if ($existingUser['status'] === 'blocked') {
                $this->handleError("This account has been blocked", 403);
            }
            $this->handleError("Email already registered", 409);
        }

      $result = $this->collection->insertOne([
    'firstName' => $data['firstName'],
    'lastName' => $data['lastName'],
    'email' => $data['email'],
    'password' => password_hash($data['password'], PASSWORD_BCRYPT),
    'createdAt' => new UTCDateTime(),
    'timezone' => 'Africa/Johannesburg',
    'status' => 'active',
    'role' => 'user',
    'lastLogin' => null,
    // Subscription fields
    'paymentStatus' => 'inactive', // Default for new users
    'lastPaymentDate' => null,     // Will be set after first payment
    'nextBillingDate' => null,     // Will be set after subscription
    'subscriptionId' => null,      // Will be set from PayFast
    'plan' => null                 // Can be 'monthly', 'annual', etc.
]);

        if ($result->getInsertedCount() === 1) {
            $this->startSession($data['email']);
            header('Location: /index.jsx');
            exit;
        }

        $this->handleError("Registration failed", 500);
    }

    private function handleSignIn(): void {
        $data = $this->validateSignInData();
        $user = $this->collection->findOne(['email' => $data['email']]);

        if (!$user) {
            $this->handleError("Invalid credentials", 401);
        }

        if ($user['status'] === 'blocked') {
            $this->handleError("Your account has been blocked", 403);
        }

        if (!password_verify($data['password'], $user['password'])) {
            $this->handleError("Invalid credentials", 401);
        }

        $this->collection->updateOne(
            ['_id' => $user['_id']],
            ['$set' => ['lastLogin' => new UTCDateTime()]]
        );

        $this->startSession($user['email']);
        header('Location: /index.jsx');
        exit;
    }

    private function handleGetActivity(): void {
        if (!isset($_SESSION['user']['email'])) {
            $this->handleError("Unauthorized", 401);
        }

        $user = $this->collection->findOne(['email' => $_SESSION['user']['email']]);
        if (!$user) {
            $this->handleError("User not found", 404);
        }

        header('Content-Type: application/json');
        echo json_encode($this->generateActivityFeedback($user));
        exit;
    }

    private function handleBlockUser(): void {
        $this->validateAdminAccess();
        
        if (empty($_POST['email'])) {
            $this->handleError("Email required", 400);
        }

        $email = filter_var($_POST['email'], FILTER_SANITIZE_EMAIL);
        $result = $this->collection->updateOne(
            ['email' => $email],
            ['$set' => ['status' => 'blocked']]
        );

        if ($result->getModifiedCount() === 0) {
            $this->handleError("User not found or already blocked", 404);
        }

        header('Content-Type: application/json');
        echo json_encode(['success' => true]);
        exit;
    }

    private function handleUnblockUser(): void {
        $this->validateAdminAccess();
        
        if (empty($_POST['email'])) {
            $this->handleError("Email required", 400);
        }

        $email = filter_var($_POST['email'], FILTER_SANITIZE_EMAIL);
        $result = $this->collection->updateOne(
            ['email' => $email],
            ['$set' => ['status' => 'active']]
        );

        if ($result->getModifiedCount() === 0) {
            $this->handleError("User not found or already active", 404);
        }

        header('Content-Type: application/json');
        echo json_encode(['success' => true]);
        exit;
    }

    private function generateActivityFeedback(array $user): array {
        if ($user['status'] === 'blocked') {
            return [
                'status' => 'blocked',
                'message' => 'Account blocked',
                'severity' => 'critical'
            ];
        }

        $lastLogin = $user['lastLogin'] ? $user['lastLogin']->toDateTime() : null;
        $now = new DateTime();
        
        if (!$lastLogin) {
            return [
                'status' => 'active',
                'lastActivity' => 'Never logged in',
                'message' => 'New user - welcome!',
                'severity' => 'info'
            ];
        }

        $interval = $now->diff($lastLogin);
        $days = $interval->days;

        $feedback = match(true) {
            $days === 0 => ['message' => "Active today!", 'severity' => 'success'],
            $days <= 3 => ['message' => "Active recently", 'severity' => 'info'],
            $days <= 7 => ['message' => "Last seen $days days ago", 'severity' => 'warning'],
            default => ['message' => "Inactive for $days days", 'severity' => 'danger']
        };

        return [
            'status' => 'active',
            'lastActivity' => $lastLogin->format('Y-m-d H:i:s'),
            'daysInactive' => $days,
            'message' => $feedback['message'],
            'severity' => $feedback['severity']
        ];
    }

    private function validateAdminAccess(): void {
        if (!isset($_SESSION['user']['email'])) {
            $this->handleError("Unauthorized", 401);
        }

        $user = $this->collection->findOne(['email' => $_SESSION['user']['email']]);
        if (!$user || $user['role'] !== 'admin') {
            $this->handleError("Admin access required", 403);
        }
    }

    private function validateSignUpData(): array {
        $required = ['fName', 'lName', 'email', 'password'];
        foreach ($required as $field) {
            if (empty($_POST[$field])) {
                $this->handleError("Missing $field", 400);
            }
        }

        $email = filter_var($_POST['email'], FILTER_SANITIZE_EMAIL);
        if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
            $this->handleError("Invalid email", 400);
        }

        if (strlen($_POST['password']) < 8) {
            $this->handleError("Password too short", 400);
        }

        return [
            'firstName' => $this->sanitizeInput($_POST['fName']),
            'lastName' => $this->sanitizeInput($_POST['lName']),
            'email' => $email,
            'password' => $_POST['password']
        ];
    }

    private function validateSignInData(): array {
        if (empty($_POST['email']) || empty($_POST['password'])) {
            $this->handleError("Email and password required", 400);
        }

        return [
            'email' => filter_var($_POST['email'], FILTER_SANITIZE_EMAIL),
            'password' => $_POST['password']
        ];
    }

    private function startSession(string $email): void {
        $_SESSION['user'] = [
            'email' => $email,
            'ip' => $_SERVER['REMOTE_ADDR'],
            'last_active' => time(),
            'user_agent' => $_SERVER['HTTP_USER_AGENT'],
            'logged_in' => true
        ];
    }

    private function sanitizeInput(string $input): string {
        return htmlspecialchars(trim($input), ENT_QUOTES, 'UTF-8');
    }

    private function handleError(string $message, int $code): void {
        http_response_code($code);
        header('Location: /index.php?error=' . urlencode($message));
        exit;
    }
}

// Process request
(new AuthHandler())->handleRequest();