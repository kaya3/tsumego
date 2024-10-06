///<reference path="page.ts"/>

namespace Pages {
    export class AttemptTsumego extends Page<TsumegoData[]> {
        private tsumego: TsumegoData[] = [];
        private index: number = 0;
        
        private view: TsumegoView;
        private readonly boardContainer: HTMLElement;
        private readonly resultContainer: HTMLElement;
        private readonly correctSound: HTMLAudioElement;
        private readonly incorrectSound: HTMLAudioElement;
        
        public constructor(app: App) {
            super(app, 'attempt_tsumego_page');
            this.view = new TsumegoView();
            
            this.boardContainer = expectElementById('attempt_board');
            this.resultContainer = expectElementById('result_container');
            this.correctSound = expectElementById('correct_sound', 'audio');
            this.incorrectSound = expectElementById('incorrect_sound', 'audio');
        }
        
        protected hydrate(): void {
            const view = this.view;
            
            view.onComplete(async win => {
                // Show a message
                let resultContainer = this.resultContainer;
                resultContainer.classList.remove('hidden');
                if(win) {
                    resultContainer.classList.add('correct');
                    resultContainer.innerText = 'Correct!';
                } else {
                    resultContainer.classList.remove('correct');
                    resultContainer.innerText = 'Incorrect';
                }
                
                // Play a sound to indicate whether the solution is correct
                let sound = win ? this.correctSound : this.incorrectSound;
                sound.currentTime = 0;
                await sound.play();
                
                // TODO: use finer grades
                const grade: Grade = win ? 'Easy' : 'Again';
                // TODO: don't split state between this class and TsumegoView
                
                // TODO: don't delay showing next tsumego in case this request is slow
                // Response will say whether to add this tsumego back into the queue
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
                
                // If this tsumego is due to be reviewed again today, add it
                // again to the end of the queue
                if(stats && stats.srsState.interval < 1) {
                    this.tsumego.push(this.tsumego[this.index]);
                }
                
                this.index++;
                setTimeout(() => {
                    if(this.index < this.tsumego.length) {
                        this.showNextTsumego();
                    } else {
                        this.app.navigateHome();
                    }
                }, 1000);
            });
            
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
        }
        
        private showNextTsumego(): void {
            const data = this.tsumego[this.index];
            const tsumego = Tsumego.fromData(data);
            this.view.setTsumego(tsumego);
            this.resultContainer.classList.add('hidden');
    }
    }
}
