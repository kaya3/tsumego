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
                    this.showReviewButtons();
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
            if(data.length === 0) {
                // We shouldn't be on this page
                console.error(`Navigated to AttemptTsumego page with empty array`);
                this.app.navigateHome();
                return;
            }
            
            this.tsumego = data;
            this.index = -1;
            this.showNextTsumego();
            hide(this.resultContainer);
        }
        
        protected onHide(): void {
            this.tsumego = [];
            this.index = 0;
            this.view.clear();
            hide(this.resultContainer);
            this.hideReviewButtons();
        }
        
        private showMessage(isWin: boolean): void {
            let resultContainer = this.resultContainer;
            show(resultContainer);
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
            const again = stats && showAgainToday(stats);
            
            const user = this.app.currentUser;
            if(user) {
                user.reviewsDoneToday++;
                // TODO: this isn't exactly right, since this review might
                // not have been due; perhaps it was reviewed early, or not
                // scheduled at all
                if(!again) {
                    if(user.reviewsDueToday > 0) {
                        user.reviewsDueToday--;
                    }
                }
            }
            
            // If this tsumego is due to be reviewed again today, add it
            // again to the end of the queue
            if(again) {
                this.tsumego.push(this.tsumego[this.index]);
            }
        }
        
        private showReviewButtons(): void {
            show(this.reviewButtonContainer);
            for(const button of this.reviewButtons) {
                button.disabled = false;
            }
        }
        
        private hideReviewButtons(): void {
            hide(this.reviewButtonContainer);
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
                hide(this.resultContainer);
                this.hideReviewButtons();
            } else {
                this.app.navigateHome();
            }
        }
    }
    
    function showAgainToday(stats: TsumegoStats): boolean {
        return (stats.learningState === 'Learning' || stats.learningState === 'Relearning')
            && stats.srsState.interval < 1;
    }
}
