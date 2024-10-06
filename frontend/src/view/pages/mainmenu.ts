namespace Pages {
    export class MainMenu extends Page<UserDetails> {
        private readonly todaySummary: HTMLElement;
        private readonly beginReviewingButton: HTMLButtonElement;
        private readonly studyRandomButton: HTMLButtonElement;
        
        public constructor(app: App) {
            super(app, 'main_menu_page');
            
            this.todaySummary = expectElementById('today_summary');
            this.beginReviewingButton = expectElementById('begin_reviewing_button', 'button');
            this.studyRandomButton = expectElementById('study_random_button', 'button');
            this.beginReviewingButton.disabled = true;
            this.studyRandomButton.disabled = true;
        }
        
        protected hydrate(): void {
            this.beginReviewingButton.addEventListener('click', async () => {
                this.hide();
                const tsumego = await API.getPendingTsumego();
                this.app.attemptTsumegoPage.show(tsumego);
            });
            this.studyRandomButton.addEventListener('click', async () => {
                this.hide();
                const tsumego = await API.getUnstudiedTsumego();
                this.app.attemptTsumegoPage.show(tsumego);
            });
        }
        
        protected onShow(data: UserDetails): void {
            if(data.reviewsDueToday > 0) {
                this.todaySummary.innerHTML = data.reviewsDoneToday > 0
                    ? `You've done <b>${pluralise(data.reviewsDoneToday, 'problem')}</b> today, and have <b>${data.reviewsDueToday}</b> left to review.`
                    : `You have <b>${pluralise(data.reviewsDueToday, 'problem')}</b> to review today.`;
                this.beginReviewingButton.disabled = false;
                this.beginReviewingButton.classList.remove('hidden');
                this.studyRandomButton.disabled = true;
                this.studyRandomButton.classList.add('hidden');
            } else {
                this.todaySummary.innerHTML = data.reviewsDoneToday
                    ? `You've done <b>${pluralise(data.reviewsDoneToday, 'problem')}</b> today.`
                    : `No reviews due today, but you can try some new problems!`;
                this.beginReviewingButton.disabled = true;
                this.beginReviewingButton.classList.add('hidden');
                this.studyRandomButton.disabled = false;
                this.studyRandomButton.classList.remove('hidden');
            }
        }
        
        protected onHide(): void {
            this.beginReviewingButton.disabled = true;
            this.beginReviewingButton.classList.add('hidden');
            this.studyRandomButton.disabled = true;
            this.studyRandomButton.classList.add('hidden');
        }
    }
}
