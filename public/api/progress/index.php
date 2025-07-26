<?php
session_name('SecureSession');
session_start([
    'cookie_lifetime' => 86400,
    'cookie_secure' => true,
    'cookie_httponly' => true,
    'cookie_samesite' => 'Strict',
    'use_strict_mode' => true
]);

header('Content-Type: application/json');

// Verify authentication
if (!isset($_SESSION['user']['id'])) {
    http_response_code(401);
    exit(json_encode(['error' => 'Unauthorized']));
}

try {
    // Connect to MongoDB (using your existing config)
    $mongo = new MongoDB\Client(
        $_ENV['MONGODB_URI'],
        [
            'tls' => true,
            'retryWrites' => true,
            'w' => 'majority'
        ]
    );
    $db = $mongo->roomie13;
    $userId = new MongoDB\BSON\ObjectId($_SESSION['user']['_id']);

    // Get last 30 workouts
    $workouts = $db->workouts->find(
        ['userId' => $userId],
        [
            'sort' => ['startTime' => -1],
            'limit' => 30,
            'projection' => [
                'exercises' => 1,
                'startTime' => 1,
                'endTime' => 1,
                'duration' => 1
            ]
        ]
    )->toArray();

    // Get personal records
    $records = $db->workouts->aggregate([
        ['$match' => ['userId' => $userId]],
        ['$unwind' => '$exercises'],
        ['$group' => [
            '_id' => '$exercises.exerciseId',
            'maxWeight' => ['$max' => '$exercises.weight'],
            'maxReps' => ['$max' => '$exercises.reps'],
            'lastDate' => ['$last' => '$endTime']
        ]]
    ])->toArray();

    echo json_encode([
        'history' => array_map(function($workout) {
            return [
                'id' => (string)$workout->_id,
                'startTime' => $workout->startTime->toDateTime()->format('c'),
                'exercises' => $workout->exercises
            ];
        }, $workouts),
        'records' => array_reduce($records, function($carry, $record) {
            $carry[$record->_id] = [
                'maxWeight' => $record->maxWeight,
                'maxReps' => $record->maxReps,
                'lastDate' => $record->lastDate->toDateTime()->format('c')
            ];
            return $carry;
        }, [])
    ]);

} catch (Exception $e) {
    error_log('Progress API Error: ' . $e->getMessage());
    http_response_code(500);
    echo json_encode(['error' => 'Failed to load progress']);
}
