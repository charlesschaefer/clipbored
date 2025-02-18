import { CommonModule } from '@angular/common';
import { Component, computed, signal, WritableSignal } from '@angular/core';
import { invoke } from "@tauri-apps/api/core";
import { Bookmark } from '../app-config.model';
import { FieldsetModule } from 'primeng/fieldset';

@Component({
  selector: 'app-bookmark-list',
  standalone: true,
  imports: [
    CommonModule,
    FieldsetModule
  ],
  templateUrl: './bookmark-list.component.html',
  styleUrl: './bookmark-list.component.css'
})
export class BookmarkListComponent {
  bookmarks: WritableSignal<Bookmark[]> = signal([]);

  constructor() {
    this.loadBookmarks();
  }

  async loadBookmarks() {
    const loadedBookmarks = await invoke<Bookmark[]>('get_bookmarks');
    if (loadedBookmarks) {
      this.bookmarks.set(loadedBookmarks);
    }
  }

  removeBookmark(index: number): void {
    this.bookmarks.update((bookmarks) => bookmarks.filter((_, i) => i !== index));
    invoke('remove_bookmark', { index: index }).then(() => {
      console.log("Bookmark removed");
    });
  }
}