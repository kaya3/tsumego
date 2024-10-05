namespace API {
    const LOGIN_ENDPOINT = '/api/login';
    const LOGOUT_ENDPOINT = '/api/logout';
    const WHO_AM_I_ENDPOINT = '/api/who_am_i';
    
    export async function login(email: string, password: string): Promise<UserDetails | null> {
        const response = await post(LOGIN_ENDPOINT, {email, password});
        
        if(response.ok) {
            return await response.json();
        } else {
            reportError('Failed login', response);
            return null;
        }
    }
    
    export async function logout(): Promise<void> {
        const response = await post(LOGOUT_ENDPOINT);
        
        if(response.ok) {
            // Await the response, but ignore the body; it is always empty
            await response.text();
        } else {
            reportError('Failed logout', response);
        }
    }
    
    export async function whoAmI(): Promise<UserDetails | null> {
        const response = await fetch(WHO_AM_I_ENDPOINT);
        
        if(response.ok) {
            return await response.json();
        } else {
            reportError('Failed whoAmI', response);
            return null;
        }
    }
}
