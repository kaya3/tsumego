///<reference path="page.ts"/>

namespace Pages {
    export class LoginForm extends Page<{email: string | null | undefined}> {
        private readonly form: HTMLFormElement;
        private readonly email: HTMLInputElement;
        private readonly password: HTMLInputElement;
        private readonly submitButton: HTMLInputElement;
        private readonly registerButton: HTMLButtonElement;
        private readonly message: HTMLElement;
        
        public constructor(app: App) {
            super(app, 'login_page');
            
            this.form = expectElementById('login_form', 'form');
            this.email = expectElementById('login_email', 'input');
            this.password = expectElementById('login_password', 'input');
            this.submitButton = expectElementById('login_submit', 'input');
            this.registerButton = expectElementById('begin_registration_button', 'button');
            this.message = expectElementById('login_message');
        }
        
        protected hydrate(): void {
            this.form.addEventListener('submit', async e => {
                e.preventDefault();
                
                const email = this.email.value.trim();
                const password = this.password.value;
                
                if(!email.includes('@')) {
                    this.message.innerText = 'Please enter your email address';
                    this.email.focus();
                    return;
                } else if(!password) {
                    this.message.innerText = 'Please enter your password';
                    this.password.focus();
                    return;
                }
                
                this.submitButton.disabled = true;
                this.registerButton.disabled = true;
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
                    this.registerButton.disabled = false;
                    
                    // Wrong password is more likely, since email is remembered
                    this.password.focus();
                }
            });
            
            this.registerButton.addEventListener('click', () => {
                const email = this.email.value.trim();
                const password = this.password.value;
                
                this.hide();
                this.app.registerPage.show({email, password});
            });
        }
        
        protected onShow(data: {email: string | null | undefined}): void {
            this.email.value = data.email ?? '';
            this.submitButton.disabled = false;
            this.registerButton.disabled = false;
            this.message.innerText = '';
            
            this.app.loggedInHeader.hide();
            (data.email ? this.password : this.email).focus();
        }
        
        protected onHide(): void {
            this.email.value = '';
            this.password.value = '';
            this.submitButton.disabled = true;
            this.registerButton.disabled = true;
            this.message.innerText = '';
        }
    }
}
