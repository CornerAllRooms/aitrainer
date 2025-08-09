<?php
declare(strict_types=1);

class MailjetMailer {
    private $client;
    private $senderEmail;
    private $senderName;

    public function __construct(array $config) {
        if (!class_exists('\Mailjet\Client')) {
            throw new RuntimeException("Mailjet SDK not installed");
        }

        $this->client = new \Mailjet\Client(
            $config['api_key'],
            $config['secret_key'],
            true,
            ['version' => 'v3.1']
        );
        $this->senderEmail = $config['sender_email'];
        $this->senderName = $config['sender_name'];
    }

    public function sendPasswordReset(string $recipientEmail, string $resetLink): bool {
        try {
            $body = [
                'Messages' => [
                    [
                        'From' => [
                            'Email' => $this->senderEmail,
                            'Name' => $this->senderName
                        ],
                        'To' => [
                            [
                                'Email' => $recipientEmail,
                                'Name' => ''
                            ]
                        ],
                        'TemplateID' => 7152538,
                        'TemplateLanguage' => true,
                        'Subject' => 'Password Reset Request',
                        'Variables' => [
                            'reset_link' => $resetLink,
                            'support_email' => 'support@cornerroom.co.za',
                            'expiration_time' => '1 hour'
                        ]
                    ]
                ]
            ];

            error_log("Mailjet payload: ".json_encode($body));

            $response = $this->client->post(\Mailjet\Resources::$Email, ['body' => $body]);

            if (!$response->success()) {
                error_log("Mailjet error: ".print_r($response->getData(), true));
                return false;
            }

            return true;
        } catch (Exception $e) {
            error_log("Mailjet exception: ".$e->getMessage());
            return false;
        }
    }
}