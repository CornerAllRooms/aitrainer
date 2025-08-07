<?php
declare(strict_types=1);

namespace App\Middleware;

use MongoDB\Client;
use Psr\Http\Message\ResponseInterface;
use Psr\Http\Server\MiddlewareInterface;
use Psr\Http\Server\RequestHandlerInterface;

class Authenticate implements MiddlewareInterface
{
    private $users;
    private $publicRoutes = ['/', '/register', '/reset-password'];

    public function __construct()
    {
        $client = new Client($_ENV['MONGODB_URI']);
        $this->users = $client->selectCollection('roomie13', 'users1');
    }

    public function process($request, RequestHandlerInterface $handler): ResponseInterface
    {
        session_start([
            'cookie_httponly' => true,
            'cookie_secure' => true
        ]);

        $path = $request->getUri()->getPath();

        // 1. Allow public routes
        if (in_array($path, $this->publicRoutes)) {
            return $handler->handle($request);
        }

        // 2. Check session
        if (empty($_SESSION['user']['email'])) {
            $_SESSION['redirect'] = $path;
            return $this->redirect('/../index.php'); // Redirect to index.php (login)
        }

        // 3. Verify user exists
        $user = $this->users->findOne([
            'email' => $_SESSION['user']['email'],
            'status' => 'active'
        ]);

        if (!$user) {
            session_destroy();
            return $this->redirect('/?error=invalid_session');
        }

        // 4. Check payment for protected routes
        if (!$this->isPublicRoute($path) && $user['paymentStatus'] !== 'active') {
            return $this->redirect('/subscribe?required=true');
        }

        return $handler->handle($request->withAttribute('user', $user));
    }

    private function isPublicRoute(string $path): bool
    {
        return in_array($path, $this->publicRoutes) || str_starts_with($path, '/../index.php');
    }

    private function redirect(string $location): ResponseInterface
    {
        return new \Laminas\Diactoros\Response\RedirectResponse($location, 302);
    }
}