///<reference path="page.ts"/>

namespace Pages {
    export class AttemptTsumego extends Page<TsumegoData[]> {
        private tsumego: TsumegoData[] = [];
        private index: number = 0;
        
        // TODO: don't split state between this class and TsumegoView
        private view: TsumegoView;
        
        private readonly boardContainer: HTMLElement;
        private readonly resultContainer: HTMLElement;
        private readonly reviewButtonContainer: HTMLElement;
        private readonly reviewButtons: readonly HTMLButtonElement[];
        private readonly correctSound: HTMLAudioElement;
        private readonly incorrectSound: HTMLAudioElement;
        
        public constructor(app: App) {
            super(app, 'attempt_tsumego_page');
            this.view = new TsumegoView();
            
            this.boardContainer = expectElementById('attempt_board');
            this.resultContainer = expectElementById('result_container');
            this.reviewButtonContainer = expectElementById('review_buttons');
            this.reviewButtons = Array.from(this.reviewButtonContainer.getElementsByTagName('button'));
            this.correctSound = expectElementById('correct_sound', 'audio');
            this.incorrectSound = expectElementById('incorrect_sound', 'audio');
        }
        
        protected hydrate(): void {
            const view = this.view;
            
            view.onComplete(async isWin => {
                // Show a message and play a sound to indicate whether the
                // solution is correct
                this.showMessage(isWin);
                await this.playSound(isWin);
                
                if(isWin) {
                    // Show review buttons
                    this.reviewButtonContainer.classList.remove('hidden');
                    for(const button of this.reviewButtons) {
                        button.disabled = false;
                    }
                } else {
                    // Submit review immediately
                    await this.submitReview('Again');
                    setTimeout(() => {
                        this.showNextTsumego();
                    }, 1000);
                }
            });
            
            for(const button of this.reviewButtons) {
                const grade = button.dataset.grade as Grade;
                
                button.addEventListener('click', async () => {
                    await this.submitReview(grade);
                    this.showNextTsumego();
                });
            }
            
            this.boardContainer.appendChild(this.view.canvas);
            function loop() {
                view.draw();
                requestAnimationFrame(loop);
            }
            loop();
        }
        
        protected onShow(data: TsumegoData[]): void {
            this.tsumego = data;
            this.index = 0;
            // TODO: ensure the array is non-empty before navigating here
            if(data.length > 0) {
                this.showNextTsumego();
            }
            this.resultContainer.classList.add('hidden');
        }
        
        protected onHide(): void {
            this.tsumego = [];
            this.index = 0;
            this.view.clear();
            this.resultContainer.classList.add('hidden');
            this.hideReviewButtons();
        }
        
        private showMessage(isWin: boolean): void {
            let resultContainer = this.resultContainer;
            resultContainer.classList.remove('hidden');
            if(isWin) {
                resultContainer.classList.add('correct');
                resultContainer.innerText = 'Correct!';
            } else {
                resultContainer.classList.remove('correct');
                resultContainer.innerText = 'Incorrect';
            }
        }
        
        private async playSound(isWin: boolean): Promise<void> {
            let sound = isWin ? this.correctSound : this.incorrectSound;
            sound.currentTime = 0;
            await sound.play();
        }
        
        private async submitReview(grade: Grade): Promise<void> {
            const stats = await API.postReview(this.tsumego[this.index].id, grade);
            
            const user = this.app.currentUser;
            if(user) {
                user.reviewsDoneToday++;
                // TODO: this isn't exactly right, since this review might
                // not have been due; perhaps it was reviewed early, or not
                // scheduled at all
                if(stats && stats?.srsState.interval >= 1) {
                    // `interval >= 1` means this tsumego is no longer due today
                    if(user.reviewsDueToday > 0) {
                        user.reviewsDueToday--;
                    }
                }
            }
            
            // TODO: response should say whether to add this tsumego back into the queue
            
            // If this tsumego is due to be reviewed again today, add it
            // again to the end of the queue
            if(stats && stats.srsState.interval < 1) {
                this.tsumego.push(this.tsumego[this.index]);
            }
        }
        
        private hideReviewButtons(): void {
            this.reviewButtonContainer.classList.add('hidden');
            for(const button of this.reviewButtons) {
                button.disabled = true;
            }
        }
        
        private showNextTsumego(): void {
            this.index++;
            if(this.index < this.tsumego.length) {
                const data = this.tsumego[this.index];
                const tsumego = Tsumego.fromData(data);
                this.view.setTsumego(tsumego);
                this.resultContainer.classList.add('hidden');
                this.hideReviewButtons();
            } else {
                this.app.navigateHome();
            }
        }
    }
}
