type Grade = 'Again' | 'Hard' | 'Good' | 'Easy'

type LearningState = 'Learning' | 'Relearning' | 'Mature' | null

interface TsumegoStats {
    readonly id: number,
    readonly userID: number,
    readonly tsumegoID: number,
    readonly lastReviewDate: string,
    readonly reviewDue: string | null,
    readonly learningState: LearningState,
    readonly srsState: SrsState,
}

interface SrsState {
    readonly numReviews: number;
    readonly streakLength: number;
    readonly interval: number;
    readonly eFactor: number;
}
