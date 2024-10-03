namespace Pages {
    export class MainMenu extends Page<User> {
        private readonly beginStudying: HTMLButtonElement;
        
        public constructor(app: App) {
            super(app, 'main_menu_page');
            
            this.beginStudying = expectElementById('begin_studying_button', 'button');
            this.beginStudying.disabled = true;
        }
        
        protected hydrate(): void {
            this.beginStudying.addEventListener('click', async () => {
                this.hide();
                await this.app.attemptTsumegoPage.fetchAndShow();
            });
        }
        
        protected onShow(data: User): void {
            this.beginStudying.disabled = false;
        }
        
        protected onHide(): void {
            this.beginStudying.disabled = true;
        }
        
    }
}
