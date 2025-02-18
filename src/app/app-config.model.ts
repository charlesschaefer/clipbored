export interface AppConfig {
    maxItems: number;
    openShortcut: string;
    bookmarkShortcut: string;
    startMinimized: boolean;
}

export interface Bookmark {
    content: string;
}