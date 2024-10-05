namespace Pages {
    export class MainMenu extends Page<UserDetails> {
        private readonly todaySummary: HTMLElement;
        private readonly beginStudying: HTMLButtonElement;
        
        public constructor(app: App) {
            super(app, 'main_menu_page');
            
            this.todaySummary = expectElementById('today_summary');
            this.beginStudying = expectElementById('begin_studying_button', 'button');
            this.beginStudying.disabled = true;
        }
        
        protected hydrate(): void {
            this.beginStudying.addEventListener('click', async () => {
                this.hide();
                await this.app.attemptTsumegoPage.fetchAndShow();
            });
        }
        
        protected onShow(data: UserDetails): void {
            if(data.reviewsDueToday > 0) {
                this.todaySummary.innerHTML = data.reviewsDoneToday > 0
                    ? `You have completed <b>${pluralise(data.reviewsDoneToday, 'problem')}</b> today, and have <b>${data.reviewsDueToday}</b> left to review.`
                    : `You have <b>${pluralise(data.reviewsDueToday, 'problem')}</b> to review today.`;
                this.beginStudying.disabled = false;
                this.beginStudying.classList.remove('hidden');
            } else {
                this.todaySummary.innerHTML = data.reviewsDoneToday
                    ? `You have completed ${pluralise(data.reviewsDoneToday, 'problem')} today.`
                    : `No reviews due today.`;
                this.beginStudying.disabled = true;
                this.beginStudying.classList.add('hidden');
            }
        }
        
        protected onHide(): void {
            this.beginStudying.disabled = true;
            this.beginStudying.classList.add('hidden');
        }
        
    }
}
