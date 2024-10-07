class App {
    public static rememberEmail(email: string): void {
        // Remember the user's email address for next time they log in
        // It's OK if this fails, so just catch and suppress
        try {
            localStorage.setItem('userEmail', email);
        } catch(_ignored) {}
    }
    
    private static recallEmail(): string | null {
        // Try to load a previously remembered email address
        // It's OK if this fails, so just catch and suppress
        try {
            return localStorage.getItem('userEmail');
        } catch(_ignored) {}
        
        return null;
    }
    
    public readonly loggedInHeader = new LoggedInHeader(this);
    
    public readonly loginPage = new Pages.LoginForm(this);
    public readonly registerPage = new Pages.Register(this);
    public readonly registerSuccessPage = new Pages.RegisterSuccess(this);
    public readonly mainMenuPage = new Pages.MainMenu(this);
    public readonly attemptTsumegoPage = new Pages.AttemptTsumego(this);
    
    public currentPage: {hide(): void} | null = null;
    public currentUser: UserDetails | null = null;
    
    public setCurrentUser(user: UserDetails | null): void {
        this.currentUser = user;
        
        if(user) {
            App.rememberEmail(user.email);
            this.loggedInHeader.show(user);
        } else {
            this.loggedInHeader.hide();
        }
    }
    
    public navigateHome(): void {
        const user = this.currentUser;
        
        this.currentPage?.hide();
        
        if(user) {
            this.mainMenuPage.show(user);
        } else {
            // Populate the login form with the previously-used email address
            let email = App.recallEmail();
            
            this.loginPage.show({email});
        }
    }
    
    public async main() {
        const user = await API.whoAmI();
        this.setCurrentUser(user);
        this.navigateHome();
    }
}
