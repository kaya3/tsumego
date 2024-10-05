///<reference path="page.ts"/>

namespace Pages {
    export class AttemptTsumego extends Page<TsumegoData[]> {
        private tsumego: TsumegoData[] = [];
        private index: number = 0;
        
        private view: TsumegoView;
        
        public constructor(app: App) {
            super(app, 'attempt_tsumego_page');
            this.view = new TsumegoView();
        }
        
        public async fetchAndShow(): Promise<void> {
            const tsumego = await API.getPendingTsumego();
            this.show(tsumego);
        }
        
        protected hydrate(): void {
            const view = this.view;
            
            view.onComplete(win => {
                console.log(win ? 'You won!' : 'You lost');
                
                // TODO: use finer grades
                const grade: Grade = win ? 'Easy' : 'Again';
                // TODO: don't split state between this class and TsumegoView
                
                // TODO: await this properly, but don't delay showing next tsumego
                // Response will say whether to add this tsumego back into the queue
                API.postReview(this.tsumego[this.index].id, grade);
                
                this.index++;
                if(this.index >= this.tsumego.length) {
                    this.app.navigateHome();
                } else {
                    setTimeout(() => this.showNextTsumego(), 1000);
                }
            });
            
            this.container.appendChild(this.view.canvas);
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
        }
        
        protected onHide(): void {
            this.tsumego = [];
            this.index = 0;
            this.view.clear();
        }
        
        private showNextTsumego(): void {
            const data = this.tsumego[this.index];
            const tsumego = Tsumego.fromData(data);
            this.view.setTsumego(tsumego);
    }
    }
}
