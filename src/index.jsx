import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './App.css'; // CSS import

// Check authentication before rendering
const checkAuth = () => {
  const token = localStorage.getItem('authToken');
  if (!token) {
    window.location.href = '/server/public/index.php'; // Redirect to PHP login
    return false;
  }
  return true;
};

// Initialize the app
if (checkAuth()) {
  const root = ReactDOM.createRoot(document.getElementById('root'));
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}