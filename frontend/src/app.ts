class App {
    public readonly loggedInHeader = new LoggedInHeader(this);
    
    public readonly loginPage = new Pages.LoginPage(this);
    public readonly attemptTsumegoPage = new Pages.AttemptTsumego(this);
    
    public currentPage: {hide(): void} | null = null;
    public currentUser: User | null = null;
    
    public setCurrentUser(user: User | null): void {
        this.currentUser = user;
        if(user) {
            this.loggedInHeader.show(user);
        } else {
            this.loggedInHeader.hide();
        }
    }
    
    public async main() {
        const user = await API.whoAmI();
        this.currentUser = user;
        
        if(!user) {
            this.loginPage.show();
        } else {
            this.loggedInHeader.show(user);
            await this.attemptTsumegoPage.fetchAndShow();
        }
    }
}
