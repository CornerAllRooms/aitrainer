// Google Sign-In callback handler
require_once __DIR__ . '/../vendor/autoload.php';
require_once __DIR__ . '/../bootstrap.php';

$client = new Google_Client();
$client->setClientId($_ENV['GOOGLE_CLIENT_ID']);
$client->setClientSecret($_ENV['GOOGLE_CLIENT_SECRET']);
$client->setRedirectUri('https://lobby.cornerroom.co.za/google-callback');
$client->addScope('email profile');

if (isset($_GET['code'])) {
    try {
        $token = $client->fetchAccessTokenWithAuthCode($_GET['code']);
        $client->setAccessToken($token);
        
        $google_oauth = new Google_Service_Oauth2($client);
        $google_account = $google_oauth->userinfo->get();
        
        // Check if user exists in MongoDB
        $user = $users->findOne([
            '$or' => [
                ['auth.google.id' => $google_account->id],
                ['profile.email' => $google_account->email]
            ]
        ]);
        
        if (!$user) {
            // Create new user
            $insertResult = $users->insertOne([
                'auth' => [
                    'google' => [
                        'id' => $google_account->id,
                        'email' => $google_account->email,
                        'token' => $token['access_token'],
                        'refreshToken' => $token['refresh_token'] ?? null
                    ]
                ],
                'profile' => [
                    'email' => $google_account->email,
                    'firstName' => $google_account->givenName,
                    'lastName' => $google_account->familyName,
                    'avatar' => $google_account->picture,
                    'verified' => true
                ],
                'timestamps' => [
                    'createdAt' => new MongoDB\BSON\UTCDateTime(),
                    'updatedAt' => new MongoDB\BSON\UTCDateTime()
                ],
                'roles' => ['user'],
                'status' => 'active'
            ]);
            
            $userId = $insertResult->getInsertedId();
        } else {
            $userId = $user['_id'];
            // Update existing user's Google token
            $users->updateOne(
                ['_id' => $userId],
                ['$set' => [
                    'auth.google.token' => $token['access_token'],
                    'auth.google.refreshToken' => $token['refresh_token'] ?? $user['auth']['google']['refreshToken'],
                    'timestamps.updatedAt' => new MongoDB\BSON\UTCDateTime()
                ]]
            );
        }
        
        // Start PHP session
        $_SESSION['user_id'] = (string)$userId;
        $_SESSION['auth_method'] = 'google';
        header('Location: /dashboard');
        
    } catch (Exception $e) {
        error_log('Google Auth Error: ' . $e->getMessage());
        header('Location: /login?error=google_auth_failed');
    }
}