document.addEventListener('DOMContentLoaded', function() {
    const signUpButton = document.getElementById('signUpButton');
    const signInButton = document.getElementById('signInButton');
    const signInForm = document.getElementById('signIn');
    const signUpForm = document.getElementById('signup');

    // Debug: Log initial state
    console.log('Initial state - SignIn:', signInForm.classList);
    console.log('Initial state - SignUp:', signUpForm.classList);

    function toggleForms(showForm, hideForm) {
        console.log('Showing:', showForm.id, 'Hiding:', hideForm.id);

        // First remove active class from both to prevent glitches
        signInForm.classList.remove('active-form');
        signUpForm.classList.remove('active-form');

        // Then add to the one we want to show
        showForm.classList.add('active-form');

        // Debug: Verify changes
        console.log('After toggle - SignIn:', signInForm.classList);
        console.log('After toggle - SignUp:', signUpForm.classList);
    }

    // Event listeners with error handling
    if (signUpButton && signInButton) {
        signUpButton.addEventListener('click', function(e) {
            e.preventDefault();
            toggleForms(signUpForm, signInForm);
        });

        signInButton.addEventListener('click', function(e) {
            e.preventDefault();
            toggleForms(signInForm, signUpForm);
        });
    } else {
        console.error('Buttons not found! Check your IDs');
    }

    // Initialize forms
    toggleForms(signInForm, signUpForm);
});
