interface UserDetails {
    readonly id: number;
    readonly email: string;
    readonly displayName: string;
    readonly isAdmin: boolean;
    reviewsDueToday: number;
    reviewsDoneToday: number;
}
