namespace TsumegoAPI {
    const TSUMEGO_ENDPOINT = 'out/all_problems.json';
    
    export async function loadAllTsumego(): Promise<Tsumego[]> {
        const response = await fetch(TSUMEGO_ENDPOINT);
        const json: {problems: object[]} = await response.json();
        return json.problems.map(Tsumego.fromJSONObject);
    }
}
