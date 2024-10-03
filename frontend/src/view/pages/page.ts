namespace Pages {
    export abstract class Page<T> {
        protected readonly app: App;
        
        protected readonly container: HTMLElement;
        private isHydrated: boolean = false;
        
        protected constructor(app: App, containerId: string) {
            this.app = app;
            this.container = expectElementById(containerId, 'div');
        }
        
        public show(data: T): void {
            this.app.currentPage = this;
            this.container.classList.remove('hidden');
            
            if(!this.isHydrated) {
                this.hydrate();
                this.isHydrated = true;
            }
            
            this.onShow(data);
        }
        
        public hide(): void {
            this.container.classList.add('hidden');
            this.onHide();
        }
        
        protected abstract hydrate(): void;
        
        protected abstract onShow(data: T): void;
        protected abstract onHide(): void;
    }
}
