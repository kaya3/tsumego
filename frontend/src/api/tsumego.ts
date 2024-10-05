namespace API {
    const GET_PENDING_ENDPOINT = '/api/get_pending';
    
    export async function getPendingTsumego(): Promise<TsumegoData[]> {
        const response = await fetch(GET_PENDING_ENDPOINT);
        
        if(response.ok) {
            const json: {problems: TsumegoData[]} = await response.json();
            return json.problems;
        } else {
            reportError('Failed to load pending tsumego', response);
            return [];
        }
    }
}
