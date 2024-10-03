namespace API {
    /**
     * Makes an HTTP POST request to the given endpoint. If a `payload` is
     * provided, it is sent with JSON encoding.
     */
    export async function post(endpoint: string, payload?: object): Promise<Response> {
        let request: RequestInit = {
            method: 'POST',
        };
        
        if(payload !== undefined) {
            request.headers = new Headers();
            request.headers.append("Content-Type", "application/json");
            request.body = JSON.stringify(payload);
        }
        
        return await fetch(endpoint, request);
    }
}
