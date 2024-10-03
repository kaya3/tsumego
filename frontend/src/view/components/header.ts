class LoggedInHeader {
    private readonly header: HTMLElement;
    private readonly welcome: HTMLElement;
    private readonly logoutButton: HTMLButtonElement;
    
    public constructor(readonly app: App) {
        this.header = expectElementById('logged_in_header');
        this.welcome = expectElementById('logged_in_header_welcome');
        this.logoutButton = expectElementById('logout_button', 'button');
        this.logoutButton.disabled = true;
        
        this.logoutButton.addEventListener('click', async () => {
            let email = this.app.currentUser?.email;
            
            // This also hides the header
            this.app.setCurrentUser(null);
            this.app.currentPage?.hide();
            
            await API.logout();
            
            this.app.loginPage.show({email});
        });
    }
    
    public show(user: User): void {
        this.header.classList.remove('hidden');
        this.welcome.innerText = `Logged in as ${user.displayName}`;
        this.logoutButton.disabled = false;
    }
    
    public hide(): void {
        this.header.classList.add('hidden');
        this.logoutButton.disabled = true;
    }
}
