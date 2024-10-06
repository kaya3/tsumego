namespace API {
    const NUM_UNSTUDIED = 5;
    
    const GET_PENDING_ENDPOINT = '/api/get_pending';
    const GET_UNSTUDIED_ENDPOINT = `/api/get_random_unstudied/${NUM_UNSTUDIED}`;
    
    export function getPendingTsumego(): Promise<TsumegoData[]> {
        return getTsumegoArray(GET_PENDING_ENDPOINT);
    }
    
    export function getUnstudiedTsumego(): Promise<TsumegoData[]> {
        return getTsumegoArray(GET_UNSTUDIED_ENDPOINT);
    }
    
    async function getTsumegoArray(endpoint: string): Promise<TsumegoData[]> {
        const response = await fetch(endpoint);
        
        if(response.ok) {
            const json: {problems: TsumegoData[]} = await response.json();
            return json.problems;
        } else {
            reportError('Failed to load tsumego data', response);
            return [];
        }
    }
}
