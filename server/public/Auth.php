<?php
session_start();
// Your existing authentication code
$_SESSION['logged_in'] = true; 
header("Location: ../"); // ← Connects to homepage
?>