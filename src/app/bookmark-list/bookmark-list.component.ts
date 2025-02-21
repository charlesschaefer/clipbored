import { CommonModule } from '@angular/common';
import { Component, computed, OnInit, signal, WritableSignal } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";
import { listen } from '@tauri-apps/api/event';
import { ButtonModule } from 'primeng/button';
import { FieldsetModule } from 'primeng/fieldset';
import { ListboxModule } from 'primeng/listbox';
import { PanelModule } from 'primeng/panel';
import { Bookmark } from '../app-config.model';

@Component({
    selector: 'app-bookmark-list',
    standalone: true,
    imports: [
        CommonModule,
        FieldsetModule,
        PanelModule,
        ListboxModule,
        ButtonModule
    ],
    templateUrl: './bookmark-list.component.html',
    styleUrl: './bookmark-list.component.css'
})
export class BookmarkListComponent implements OnInit {
    bookmarks: WritableSignal<Bookmark[]> = signal([]);

    constructor() {
        
    }

    ngOnInit(): void {
        this.loadBookmarks();

        listen("bookmarks-updated", () => this.loadBookmarks());
    }

    async loadBookmarks() {
        const loadedBookmarks = await invoke<Bookmark[]>('get_bookmarks');
        if (loadedBookmarks && loadedBookmarks.length > 0) {
            this.bookmarks.set(loadedBookmarks);
        }

        console.log("Bookmarks loaded", this.bookmarks());
    }

    removeBookmark(item: string): void {
        let index;
        this.bookmarks.update((bookmarks) => bookmarks.filter((txt, idx) => {
            if (txt.content === item) {
                index = idx;
            }
            return txt.content !== item
        }));
        invoke('remove_bookmark', { index }).then(() => {
            console.log("Bookmark removed");
        });
    }

    toggleBookmark(item: string) {
        invoke('toggle_bookmark', { content: item });
    }

}
