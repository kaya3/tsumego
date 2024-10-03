///<reference path="page.ts"/>

namespace Pages {
    export class LoginPage extends Page<void> {
        private readonly email: HTMLInputElement;
        private readonly password: HTMLInputElement;
        private readonly submitButton: HTMLInputElement;
        private readonly message: HTMLElement;
        
        public constructor(app: App) {
            super(app, 'login_page');
            
            this.email = expectElementById('login_email', 'input');
            this.password = expectElementById('login_password', 'input');
            this.submitButton = expectElementById('login_submit', 'input');
            this.message = expectElementById('login_message');
        }
        
        protected hydrate(): void {
            this.submitButton.addEventListener('click', async e => {
                this.submitButton.disabled = true;
                
                const email = this.email.value;
                const password = this.password.value;
                
                const user = await API.login(email, password);
                if(user) {
                    // This also shows the header
                    this.app.setCurrentUser(user);
                    
                    // Go to tsumego attempt page
                    this.hide();
                    await this.app.attemptTsumegoPage.fetchAndShow();
                } else {
                    this.message.innerText = 'Incorrect email or password';
                    this.submitButton.disabled = false;
                }
                
                e.preventDefault();
                return false;
            });
        }
        
        protected onShow(): void {
            this.submitButton.disabled = false;
            this.app.loggedInHeader.hide();
        }
        
        protected onHide(): void {
            this.email.value = '';
            this.password.value = '';
            this.submitButton.disabled = true;
            this.message.innerText = '';
        }
    }
}
