class LoggedInHeader {
    private readonly header: HTMLElement;
    private readonly logoutButton: HTMLButtonElement;
    
    public constructor(readonly app: App) {
        this.header = expectElementById('logged_in_header');
        this.logoutButton = expectElementById('logout_button', 'button');
        this.logoutButton.disabled = true;
        
        this.logoutButton.addEventListener('click', async () => {
            // This also hides the header
            this.app.setCurrentUser(null);
            this.app.currentPage?.hide();
            
            await API.logout();
            
            this.app.loginPage.show();
        });
    }
    
    public show(user: User): void {
        this.header.classList.remove('hidden');
        this.logoutButton.disabled = false;
    }
    
    public hide(): void {
        this.header.classList.add('hidden');
        this.logoutButton.disabled = true;
    }
}
