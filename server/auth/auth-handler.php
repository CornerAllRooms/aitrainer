<?php
declare(strict_types=1);

require __DIR__.'/bootstrap.php';

final class AuthHandler {
    private $collection;
    
    public function __construct(MongoDB\Client $mongoClient) {
        $this->collection = $mongoClient->selectCollection('roomie13', 'users1');
    }

    public function handleRequest(): void {
        header('Content-Type: application/json');
        
        try {
            match(true) {
                ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['signUp'])) => $this->handleSignUp(),
                ($_SERVER['REQUEST_METHOD'] === 'POST' && isset($_POST['signIn'])) => $this->handleSignIn(),
                default => throw new RuntimeException('Invalid request', 400)
            };
        } catch (Exception $e) {
            http_response_code($e->getCode() ?: 500);
            exit(json_encode(['error' => $e->getMessage()]));
        }
    }

    private function handleSignUp(): void {
        $data = $this->validateSignUpData();
        
        if ($this->collection->findOne(['email' => $data['email']])) {
            throw new RuntimeException('Email already exists', 409);
        }

        $this->collection->insertOne([
            'firstName' => $data['firstName'],
            'lastName' => $data['lastName'],
            'email' => $data['email'],
            'password' => password_hash($data['password'], PASSWORD_BCRYPT),
            'createdAt' => new MongoDB\BSON\UTCDateTime(),
            'lastLogin' => null,
            'failedAttempts' => 0,
            'status' => 'active'
        ]);

        $this->createSession($data['email']);
        echo json_encode(['success' => true]);
    }

    private function handleSignIn(): void {
        $email = $this->sanitizeEmail($_POST['email']);
        $user = $this->collection->findOne(['email' => $email]);

        if (!$user || !password_verify($_POST['password'], $user['password'])) {
            if ($user) {
                $this->collection->updateOne(
                    ['_id' => $user['_id']],
                    ['$inc' => ['failedAttempts' => 1]]
                );
            }
            throw new RuntimeException('Invalid credentials', 401);
        }

        $this->createSession($email);
        $this->collection->updateOne(
            ['_id' => $user['_id']],
            ['$set' => ['lastLogin' => new MongoDB\BSON\UTCDateTime()]]
        );

        echo json_encode(['success' => true]);
    }

    private function createSession(string $email): void {
        session_regenerate_id(true);
        $_SESSION = [
            'user_id' => (string)$user['_id'] ?? '',
            'email' => $email,
            'ip' => $_SERVER['REMOTE_ADDR'],
            'ua' => $_SERVER['HTTP_USER_AGENT'],
            'created_at' => time(),
            'csrf_token' => bin2hex(random_bytes(32))
        ];
    }

    private function validateSignUpData(): array {
        $required = ['fName', 'lName', 'email', 'password', 'confirmPassword'];
        foreach ($required as $field) {
            if (empty($_POST[$field])) {
                throw new RuntimeException("Missing field: $field", 400);
            }
        }

        if ($_POST['password'] !== $_POST['confirmPassword']) {
            throw new RuntimeException('Passwords do not match', 400);
        }

        return [
            'firstName' => $this->sanitizeInput($_POST['fName']),
            'lastName' => $this->sanitizeInput($_POST['lName']),
            'email' => $this->sanitizeEmail($_POST['email']),
            'password' => $_POST['password']
        ];
    }

    private function sanitizeEmail(string $email): string {
        $email = filter_var($email, FILTER_SANITIZE_EMAIL);
        if (!filter_var($email, FILTER_VALIDATE_EMAIL)) {
            throw new RuntimeException('Invalid email', 400);
        }
        return $email;
    }

    private function sanitizeInput(string $input): string {
        return htmlspecialchars(strip_tags(trim($input)), ENT_QUOTES, 'UTF-8');
    }
}

(new AuthHandler($mongoClient))->handleRequest();