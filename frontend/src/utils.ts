/**
 * Helper function to throw an error on unrecoverable failure. Any additional
 * arguments will be logged to the console for debugging purposes.
 * 
 * Unlike a `throw` statement, this function can be used conveniently within an
 * expression.
 */
function fail(message: string, ...args: unknown[]): never {
    if(args.length > 0) {
        console.error(message, args);
    }
    throw new Error(message);
}

/**
 * Helper function to get an element from the DOM by its ID, or throw an error
 * if no such element exists.
 * 
 * If `expectedTagName` is provided, and does not match the the tag's actual
 * name, an error is thrown.
 */
function expectElementById<T extends keyof HTMLElementTagNameMap>(id: string, expectedTagName: T): HTMLElementTagNameMap[T];
function expectElementById(id: string): HTMLElement;
function expectElementById(id: string, tag?: string): HTMLElement {
    const element = document.getElementById(id)
        ?? fail(`No such element with id '${id}'`);
    
    if(tag && element.tagName.toLowerCase() !== tag) {
        fail(`Expected element with id '${id}' to have tag name '${tag}'; was '${element.tagName}'`);
    }
    
    return element;
}

/**
 * Returns a string like `"1 problem"` or `"5 problems"`, given the count and
 * the singular noun. The plural form of the noun can be given as an optional
 * argument; by default, it is the singular noun followed by the letter 's'.
 * 
 * This technique only really works for simple sentences in English, but it is
 * unlikely this app will support other languages any time soon.
 */
function pluralise(count: number, noun: string, nounPlural?: string): string {
    nounPlural ??= noun + 's';
    return `${count} ${count === 1 ? noun : nounPlural}`;
}
