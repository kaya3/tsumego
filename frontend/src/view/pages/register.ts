namespace Pages {
    export interface RegisterData {
        readonly email: string;
        readonly password: string;
    }
    
    export class RegisterPage extends Page<RegisterData> {
        private readonly form: HTMLFormElement;
        private readonly email: HTMLInputElement;
        private readonly displayName: HTMLInputElement;
        private readonly password: HTMLInputElement;
        private readonly confirmPassword: HTMLInputElement;
        private readonly submitButton: HTMLInputElement;
        private readonly message: HTMLElement;
        
        public constructor(app: App) {
            super(app, 'register_page');
            
            this.form = expectElementById('register_form', 'form');
            this.email = expectElementById('register_email', 'input');
            this.displayName = expectElementById('register_display_name', 'input');
            this.password = expectElementById('register_password', 'input');
            this.confirmPassword = expectElementById('register_confirm_password', 'input');
            this.submitButton = expectElementById('register_submit', 'input');
            this.message = expectElementById('register_message');
        }
        
        protected hydrate(): void {
            this.form.addEventListener('submit', async e => {
                e.preventDefault();
                
                const email = this.email.value.trim();
                const displayName = this.displayName.value.trim();
                const password = this.password.value;
                const confirmPassword = this.confirmPassword.value;
                
                if(!email.includes('@')) {
                    this.message.innerText = 'Please enter your email address';
                    this.email.focus();
                    return;
                } else if(!displayName) {
                    this.message.innerText = 'Please choose a display name';
                    this.displayName.focus();
                    return;
                } else if(!password) {
                    this.message.innerText = 'Please choose a password';
                    this.password.focus();
                    return;
                } else if(password !== confirmPassword) {
                    this.message.innerText = 'Passwords do not match';
                    this.confirmPassword.focus();
                    return;
                }
                
                this.submitButton.disabled = true;
                
                const outcome = await API.register(email, displayName, password);
                if(outcome && !outcome.error) {
                    // Remember the user's email address for when they log in
                    App.rememberEmail(email);
                    
                    // Navigate to success page
                    this.hide();
                    this.app.registerSuccessPage.show(outcome);
                } else {
                    this.submitButton.disabled = false;
                    this.message.innerText = outcome?.error
                        ?? 'Unknown error occurred; try again later';
                    
                    if(outcome?.error?.toLowerCase().includes('email')) {
                        this.email.focus();
                    }
                }
            });
        }
        
        protected onShow(data: RegisterData): void {
            this.email.value = data.email;
            this.password.value = data.password;
            this.submitButton.disabled = false;
            this.message.innerText = '';
            
            (data.email ? this.displayName : this.email).focus();
        }
        
        protected onHide(): void {
            this.email.value = '';
            this.displayName.value = '';
            this.password.value = '';
            this.confirmPassword.value = '';
            this.submitButton.disabled = true;
            this.message.innerText = '';
        }
    }
    
    export class RegisterSuccessPage extends Page<{verificationID: number}> {
        public constructor(app: App) {
            super(app, 'register_success_page');
        }
        
        protected hydrate(): void {
            // Do nothing
        }
        
        protected onShow(data: {verificationID: number}): void {
            // Do nothing
        }
        
        protected onHide(): void {
            // Do nothing
        }
    }
}
