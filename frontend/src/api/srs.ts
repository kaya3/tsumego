namespace API {
    const POST_REVIEW_ENDPOINT = '/api/review';
    
    /**
     * Posts the grade for a review of the given tsumego, returning the new
     * learning stats for this tsumego.
     * 
     * Returns `null` if an error occurs while posting the review.
     */
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
