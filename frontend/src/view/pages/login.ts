///<reference path="page.ts"/>

namespace Pages {
    export class LoginForm extends Page<{email: string | null | undefined}> {
        private readonly form: HTMLFormElement;
        private readonly email: HTMLInputElement;
        private readonly password: HTMLInputElement;
        private readonly submitButton: HTMLInputElement;
        private readonly message: HTMLElement;
        
        public constructor(app: App) {
            super(app, 'login_page');
            
            this.form = expectElementById('login_form', 'form');
            this.email = expectElementById('login_email', 'input');
            this.password = expectElementById('login_password', 'input');
            this.submitButton = expectElementById('login_submit', 'input');
            this.message = expectElementById('login_message');
        }
        
        protected hydrate(): void {
            this.form.addEventListener('submit', async e => {
                e.preventDefault();
                
                this.submitButton.disabled = true;
                
                const email = this.email.value;
                const password = this.password.value;
                
                const user = await API.login(email, password);
                if(user) {
                    // This also shows the header
                    this.app.setCurrentUser(user);
                    
                    // Go to main menu
                    this.hide();
                    this.app.mainMenuPage.show(user);
                } else {
                    this.message.innerText = 'Incorrect email or password';
                    this.submitButton.disabled = false;
                }
                
                return false;
            });
        }
        
        protected onShow(params: {email: string | null | undefined}): void {
            this.email.value = params.email ?? '';
            this.submitButton.disabled = false;
            this.app.loggedInHeader.hide();
            
            (params.email ? this.password : this.email).focus();
        }
        
        protected onHide(): void {
            this.email.value = '';
            this.password.value = '';
            this.submitButton.disabled = true;
            this.message.innerText = '';
        }
    }
}
