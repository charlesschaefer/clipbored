import { Component, OnInit, signal } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event'; // Import listen
import { ButtonModule } from 'primeng/button';
import { ListboxModule } from 'primeng/listbox';
import { PanelModule } from 'primeng/panel';

@Component({
    selector: 'app-clipboard-list',
    imports: [
        PanelModule,
        ListboxModule,
        ButtonModule
    ],
    templateUrl: './clipboard-list.component.html',
    styleUrl: './clipboard-list.component.scss'
})
export class ClipboardListComponent implements OnInit{
    
    clipboardItems = signal([
        'Sample clipboard item 1',
        'Sample clipboard item 2', 
        'Sample clipboard item 3'
    ]);

    ngOnInit() {
        this.loadClipboardItems();

        // Listen for the clipboard-updated event
        listen('clipboard-updated', () => {
            this.loadClipboardItems(); // Reload items on event
        })
    }

    async loadClipboardItems() {
        const items = await invoke<string[]>('get_clipboard_items');
        this.clipboardItems.set(items);
    }

    toggleBookmark(item: string) {
        invoke('toggle_bookmark', { content: item });
    }

    deleteItem(item: string) {
        invoke('delete_clipboard_item', { item }).then(() => {
            this.loadClipboardItems();
        });
    }
}
