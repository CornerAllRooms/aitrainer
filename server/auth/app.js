require('dotenv').config(); // Load environment variables
const express = require('express');
const { OAuth2Client } = require('google-auth-library');
const cors = require('cors');
const bodyParser = require('body-parser');

const app = express();
app.use(cors()); // Allow frontend requests
app.use(bodyParser.json());

const client = new OAuth2Client(process.env.GOOGLE_CLIENT_ID); // From .env

app.post('/auth/google', async (req, res) => {
    const { token } = req.body;
    try {
        const ticket = await client.verifyIdToken({
            idToken: token,
            audience: process.env.GOOGLE_CLIENT_ID, // From .env
        });
        const payload = ticket.getPayload();
        console.log('Google user data:', payload);
        
        // Save user to DB or create a session
        // Example: Store in session
        req.session.user = payload; // Requires express-session
        
        res.json({ success: true, user: payload });
    } catch (error) {
        console.error('Google token verification failed:', error);
        res.status(400).json({ success: false, error: 'Invalid token' });
    }
});

const PORT = process.env.PORT || 3000; // Fallback to 3000 if PORT is not set
app.listen(PORT, () => console.log(`Server running on process.env.MONGODB_URI`));