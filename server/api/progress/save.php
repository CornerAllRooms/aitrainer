<?php
session_name('SecureSession');
session_start([
    'cookie_lifetime' => 86400,
    'cookie_secure' => true,
    'cookie_httponly' => true,
    'cookie_samesite' => 'Strict'
]);

header('Content-Type: application/json');

// Verify CSRF token
if ($_SERVER['REQUEST_METHOD'] !== 'POST' || 
    empty($_SERVER['HTTP_X_CSRF_TOKEN']) || 
    $_SERVER['HTTP_X_CSRF_TOKEN'] !== $_SESSION['csrf_token']) {
    http_response_code(403);
    exit(json_encode(['error' => 'Invalid CSRF token']));
}

// Verify authentication
if (!isset($_SESSION['user']['_id'])) {
    http_response_code(401);
    exit(json_encode(['error' => 'Unauthorized']));
}

try {
    $input = json_decode(file_get_contents('php://input'), true);
    $mongo = new MongoDB\Client($_ENV['MONGODB_URI']);
    $db = $mongo->roomie13;
    $userId = new MongoDB\BSON\ObjectId($_SESSION['user']['_id']);

    // Prepare workout document
    $workout = [
        'userId' => $userId,
        'startTime' => new MongoDB\BSON\UTCDateTime(strtotime($input['startTime']) * 1000),
        'endTime' => new MongoDB\BSON\UTCDateTime(),
        'exercises' => array_map(function($exercise) {
            return [
                'exerciseId' => $exercise['exerciseId'],
                'name' => $exercise['name'],
                'sets' => $exercise['sets'],
                'maxWeight' => max(array_column($exercise['sets'], 'weight')),
                'totalReps' => array_sum(array_column($exercise['sets'], 'reps'))
            ];
        }, $input['exercises'])
    ];

    // Insert into MongoDB
    $result = $db->workouts->insertOne($workout);

    echo json_encode([
        'success' => true,
        'workoutId' => (string)$result->getInsertedId()
    ]);

} catch (Exception $e) {
    error_log('Workout Save Error: ' . $e->getMessage());
    http_response_code(500);
    echo json_encode(['error' => 'Failed to save workout']);
}
