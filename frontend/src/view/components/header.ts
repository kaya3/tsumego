class LoggedInHeader {
    private readonly header: HTMLElement;
    private readonly username: HTMLElement;
    private readonly logoutButton: HTMLButtonElement;
    
    public constructor(readonly app: App) {
        this.header = expectElementById('logged_in_header');
        this.username = expectElementById('header_username');
        this.logoutButton = expectElementById('logout_button', 'button');
        this.logoutButton.disabled = true;
        
        this.logoutButton.addEventListener('click', async () => {
            this.logoutButton.disabled = true;
            this.app.currentPage?.hide();
            
            // Remember email, in order to show it in the login form
            let email = this.app.currentUser?.email;
            await API.logout();
            
            // This also hides the header
            this.app.setCurrentUser(null);
            this.app.loginPage.show({email});
        });
    }
    
    public show(user: User): void {
        this.header.classList.remove('hidden');
        this.username.innerText = user.displayName;
        this.logoutButton.disabled = false;
    }
    
    public hide(): void {
        this.header.classList.add('hidden');
        this.logoutButton.disabled = true;
    }
}
