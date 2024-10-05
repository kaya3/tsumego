namespace API {
    const ALL_PROBLEMS_ENDPOINT = '/api/all_problems';
    
    export async function loadAllTsumego(): Promise<TsumegoData[]> {
        const response = await fetch(ALL_PROBLEMS_ENDPOINT);
        
        if(response.ok) {
            const json: {problems: TsumegoData[]} = await response.json();
            return json.problems;
        } else {
            reportError('Failed to load tsumego', response);
            return [];
        }
    }
}
