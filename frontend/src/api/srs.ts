namespace API {
    const POST_REVIEW_ENDPOINT = '/api/review';
    
    export async function postReview(tsumegoID: number, grade: Grade): Promise<TsumegoStats | null> {
        const response = await post(POST_REVIEW_ENDPOINT, {tsumegoID, grade});
        
        if(response.ok) {
            return await response.json();
        } else {
            reportError('Failed to post review', response);
            return null;
        }
    }
}
