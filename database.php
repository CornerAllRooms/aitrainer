<?php
$config = require __DIR__.'/settings.php';
return $config;
$config = [
    'mongo_uri' => 'mongodb+srv://Molapi:'.urlencode('131296310').'@cluster0.eo47vdh.mongodb.net/yourdbname?retryWrites=true&w=majority',
    'options' => [
        'tls' => true,
        'tlsAllowInvalidCertificates' => false,
        'tlsCAFile' => '/etc/ssl/certs/ca-certificates.crt', // Standard CA path
        'serverSelectionTimeoutMS' => 5000,
        'socketTimeoutMS' => 30000
    ]
];