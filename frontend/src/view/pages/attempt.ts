///<reference path="page.ts"/>

namespace Pages {
    export class AttemptTsumego extends Page<Tsumego[]> {
        private tsumego: Tsumego[] = [];
        private index: number = 0;
        
        private view: TsumegoView;
        
        public constructor(app: App) {
            super(app, 'attempt_tsumego_page');
            this.view = new TsumegoView();
        }
        
        public async fetchAndShow(): Promise<void> {
            const tsumego = await API.loadAllTsumego();
            this.show(tsumego);
        }
        
        protected hydrate(): void {
            const view = this.view;
            
            view.onComplete(win => {
                console.log(win ? 'You won!' : 'You lost');
                
                this.index = (this.index + 1) % this.tsumego.length;
                setTimeout(() => view.setTsumego(this.tsumego[this.index]), 1000);
            });
            
            this.container.appendChild(this.view.canvas);
            function loop() {
                view.draw();
                requestAnimationFrame(loop);
            }
            loop();
        }
        
        protected onShow(data: Tsumego[]): void {
            this.tsumego = data;
            this.index = 0;
            if(data.length > 0) {
                this.view.setTsumego(data[0]);
            }
        }
        
        protected onHide(): void {
            this.tsumego = [];
            this.index = 0;
            this.view.clear();
        }
    }
}
