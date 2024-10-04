namespace API {
    const ALL_PROBLEMS_ENDPOINT = '/api/all_problems';
    
    export async function loadAllTsumego(): Promise<Tsumego[]> {
        const response = await fetch(ALL_PROBLEMS_ENDPOINT);
        
        if(response.ok) {
            const json: {problems: object[]} = await response.json();
            return json.problems.map(Tsumego.fromJSONObject);
        } else {
            reportError('Failed to load tsumego', response);
            return [];
        }
    }
}
