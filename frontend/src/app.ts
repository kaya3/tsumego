class App {
    public readonly loggedInHeader = new LoggedInHeader(this);
    
    public readonly loginPage = new Pages.LoginForm(this);
    public readonly mainMenuPage = new Pages.MainMenu(this);
    public readonly attemptTsumegoPage = new Pages.AttemptTsumego(this);
    
    public currentPage: {hide(): void} | null = null;
    public currentUser: User | null = null;
    
    public setCurrentUser(user: User | null): void {
        this.currentUser = user;
        
        if(user) {
            // Remember the user's email address for next time they log in
            try {
                localStorage.setItem('userEmail', user.email);
            } catch(_ignored) {}
            
            this.loggedInHeader.show(user);
        } else {
            this.loggedInHeader.hide();
        }
    }
    
    public async main() {
        const user = await API.whoAmI();
        this.setCurrentUser(user);
        
        if(!user) {
            // Populate the login form with the previously-used email address
            let email: string | null = null;
            try {
                email = localStorage.getItem('userEmail');
            } catch(_ignored) {}
            
            this.loginPage.show({email});
        } else {
            this.mainMenuPage.show(user);
        }
    }
}
