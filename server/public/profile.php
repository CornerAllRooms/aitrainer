<?php
// Start session and check authentication
session_start();
if (!isset($_SESSION['user']['email'])) {
    header('Location: /index.php');
    exit;
}

// Database connection
require_once __DIR__.'/../config/database.php';
$client = new MongoDB\Client($_ENV['MONGODB_URI']);
$collection = $client->selectCollection('roomie13', 'users');

// Get user data
$user = $collection->findOne(['email' => $_SESSION['user']['email']]);

if (!$user) {
    die('User not found');
}

// Calculate activity status
$lastLogin = isset($user['lastLogin']) ? $user['lastLogin']->toDateTime() : null;
$now = new DateTime();
$daysInactive = $lastLogin ? $now->diff($lastLogin)->days : null;

// Determine status
if ($user['status'] === 'blocked') {
    $statusClass = 'danger';
    $statusText = 'Blocked';
    $statusMessage = 'Account suspended';
} elseif (!$lastLogin) {
    $statusClass = 'info';
    $statusText = 'New';
    $statusMessage = 'Welcome to your new account!';
} elseif ($daysInactive === 0) {
    $statusClass = 'success';
    $statusText = 'Active';
    $statusMessage = 'All systems operational';
} elseif ($daysInactive <= 3) {
    $statusClass = 'success';
    $statusText = 'Active';
    $statusMessage = 'Recently active';
} elseif ($daysInactive <= 7) {
    $statusClass = 'warning';
    $statusText = 'Inactive';
    $statusMessage = 'Last seen '.$daysInactive.' days ago';
} else {
    $statusClass = 'danger';
    $statusText = 'Inactive';
    $statusMessage = 'Long time no see!';
}

// Format last active date
$lastActiveFormatted = $lastLogin ? $lastLogin->format('F j, Y g:i A') : 'Never';
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title><?php echo htmlspecialchars($user['firstName'] ?? 'User'); ?>'s Profile</title>
    <link rel="stylesheet" href="/assets/profile.css">
    <link rel="preconnect" href="https://fonts.googleapis.com">
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600&display=swap" rel="stylesheet">
</head>
<body>
    <div class="profile-container">
        <div class="profile-header">
            <div class="avatar" style="background: linear-gradient(135deg, #<?php echo substr(md5($user['email']), 0, 6); ?>, #<?php echo substr(md5($user['email']), 6, 6); ?>)">
                <?php echo strtoupper(substr($user['firstName'] ?? 'U', 0, 1)); ?><div class="subscription-card <?php echo $subscriptionActive ? 'active' : 'inactive'; ?>">
    <h3>Subscription Management</h3>
    
    <?php if ($subscriptionActive): ?>
        <div class="subscription-details">
            <p><strong>Status:</strong> <span class="status-badge active">Active</span></p>
            <p><strong>Last Payment:</strong> <?php echo $lastPaymentDate; ?></p>
            <p><strong>Next Billing:</strong> 
                <?php 
                if ($user['subscriptionType'] === 'recurring') {
                    echo date('F j, Y', strtotime($lastPaymentDate . ' +1 month'));
                } else {
                    echo 'Non-recurring';
                }
                ?>
            </p>
        </div>
        
        <form id="cancelSubscriptionForm" action="/api/cancel_subscription.php" method="POST">
            <input type="hidden" name="csrf_token" value="<?php echo $_SESSION['csrf_token']; ?>">
            <button type="submit" class="cancel-button">
                Cancel Subscription
            </button>
        </form>
        
        <div class="cancellation-warning">
            <p>⚠️ You'll retain access until the end of your billing period.</p>
        </div>
    <?php else: ?>
        <div class="subscription-details">
            <p><strong>Status:</strong> <span class="status-badge inactive">Inactive</span></p>
            <a href="/subscribe.php" class="subscribe-button">Subscribe Now</a>
        </div>
    <?php endif; ?>
</div>
            </div>
            <h1>Hello, <span class="username"><?php echo htmlspecialchars($user['firstName'] ?? 'User'); ?></span>!</h1>
            <p class="subtitle">Here's your current activity status</p>
        </div>

        <div class="activity-card <?php echo $statusClass; ?>">
            <div class="activity-icon">
                <svg viewBox="0 0 100 100" class="icon-circle">
                    <circle cx="50" cy="50" r="45" fill="none"/>
                </svg>
                <svg viewBox="0 0 100 100" class="icon-mark" data-type="<?php echo $statusClass; ?>">
                    <?php if ($statusClass === 'success'): ?>
                        <path d="M20,50 L40,70 L80,30" fill="none"/>
                    <?php elseif ($statusClass === 'warning'): ?>
                        <path d="M50,10 L90,90 L10,90 Z" fill="none"/>
                        <text x="50" y="65" text-anchor="middle" font-size="40">!</text>
                    <?php elseif ($statusClass === 'danger'): ?>
                        <path d="M20,20 L80,80 M80,20 L20,80" fill="none"/>
                    <?php else: ?>
                        <circle cx="50" cy="30" r="10" fill="none"/>
                        <path d="M50,45 L50,80" fill="none"/>
                    <?php endif; ?>
                </svg>
            </div>
            <div class="activity-details">
                <h3>Account Status</h3>
                <div class="status-grid">
                    <div class="status-item">
                        <span class="label">Current:</span>
                        <span class="status-badge"><?php echo $statusText; ?></span>
                    </div>
                    <div class="status-item">
                        <span class="label">Last Active:</span>
                        <span class="value"><?php echo $lastActiveFormatted; ?></span>
                    </div>
                    <div class="status-item full-width">
                        <span class="label">Message:</span>
                        <span class="activity-message"><?php echo $statusMessage; ?></span>
                    </div>
                </div>
            </div>
        </div>

        <div class="stats-grid">
            <div class="stat-card">
                <div class="stat-icon">📅</div>
                <div class="stat-value">
                    <?php echo $user['daysActive'] ?? '0'; ?>
                </div>
                <div class="stat-label">Days Active</div>
            </div>
            <div class="stat-card">
                <div class="stat-icon">🔥</div>
                <div class="stat-value">
                    <?php echo $user['currentStreak'] ?? '0'; ?>
                </div>
                <div class="stat-label">Day Streak</div>
            </div>
        </div>
    </div>

    <script>
        // Dynamic icon loader
        document.addEventListener('DOMContentLoaded', () => {
            const cards = document.querySelectorAll('.activity-card');
            
            cards.forEach(card => {
                const type = card.classList.contains('success') ? 'success' :
                            card.classList.contains('warning') ? 'warning' :
                            card.classList.contains('danger') ? 'danger' : 'info';
                
                const icon = card.querySelector(`.icon-mark[data-type="${type}"]`);
                if (icon) {
                    icon.style.display = 'block';
                    // Trigger animation restart
                    void icon.offsetWidth;
                }
            });
        });
    </script>
</body>
</html>