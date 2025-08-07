import { create } from 'zustand';
import { persist } from 'zustand/middleware';

const useUserStore = create(
  persist(
    (set, get) => ({
      // Session state (mirrors PHP session)
      isAuthenticated: false,
      user: null,
      csrfToken: null,

      // Initialize from PHP session
      initFromSession: async () => {
        try {
          const response = await fetch('/session-status.php', {
            credentials: 'include' // Send cookies
          });
          const data = await response.json();

          if (data.authenticated) {
            set({
              isAuthenticated: true,
              user: data.user,
              csrfToken: data.csrf_token
            });
          }
          return data.authenticated;
        } catch (error) {
          console.error('Session check failed:', error);
          return false;
        }
      },

      // Login (using your existing PHP endpoint)
      login: async (email, password) => {
        const formData = new FormData();
        formData.append('email', email);
        formData.append('password', password);
        formData.append('csrf_token', get().csrfToken);

        try {
          const response = await fetch('/../../server/public/index.php', {
            method: 'POST',
            body: formData,
            credentials: 'include'
          });
          
          if (response.redirected) {
            return window.location.href = response.url;
          }

          const result = await response.json();
          if (result.success) {
            set({ isAuthenticated: true, user: result.user });
            return true;
          }
          return false;
        } catch (error) {
          console.error('Login failed:', error);
          return false;
        }
      },

      // Logout (using your existing PHP endpoint)
      logout: async () => {
        await fetch('/../../server/public/logout.php', {
          method: 'POST',
          credentials: 'include'
        });
        set({ isAuthenticated: false, user: null });
        window.location.reload();
      },

      // Verify CSRF token matches PHP session
      verifyCSRF: (token) => {
        return token === get().csrfToken;
      }
    }),
    {
      name: 'user-store',
      storage: {
        getItem: async () => null, // Disable localStorage persistence
        setItem: async () => {},   // Keep everything in PHP session
        removeItem: async () => {}
      }
    }
  )
);

// Initialize on store creation
useUserStore.getState().initFromSession();

export default useUserStore;