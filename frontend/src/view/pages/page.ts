namespace Pages {
    /**
     * Represents a page which can be shown in the client; a page, plus
     * possibly the logged-in header, forms the user interface.
     */
    export abstract class Page<T> {
        protected readonly app: App;
        
        /**
         * The `div` element for this page in the DOM.
         */
        protected readonly container: HTMLElement;
        
        /**
         * Whether or not `this.hydrate()` has been invoked yet.
         */
        private isHydrated: boolean = false;
        
        protected constructor(app: App, containerID: string) {
            this.app = app;
            this.container = expectElementById(containerID, 'div');
        }
        
        /**
         * Navigate to this page, initialising it with the given data and
         * making it visible. When navigating away from another page, it is the
         * caller's responsibility to invoke `.hide()` on the previous page.
         */
        public show(data: T): void {
            this.app.currentPage = this;
            show(this.container);
            
            if(!this.isHydrated) {
                this.hydrate();
                this.isHydrated = true;
            }
            
            this.onShow(data);
        }
        
        /**
         * Hides this page. Should be invoked when navigating away from this
         * page to another page.
         */
        public hide(): void {
            hide(this.container);
            this.onHide();
        }
        
        /**
         * Registers event handlers for interactive elements on this page.
         * Called once, the first time this page is shown.
         */
        protected abstract hydrate(): void;
        
        /**
         * Called when this page is navigated to. Should set up the state of
         * the page in the DOM.
         */
        protected abstract onShow(data: T): void;
        
        /**
         * Called when this page is navigated away from. Should reset the state
         * of the page in the DOM, including disabling interactive elements.
         */
        protected abstract onHide(): void;
    }
}
