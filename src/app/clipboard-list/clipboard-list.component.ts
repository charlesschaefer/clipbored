import { Component, OnInit } from '@angular/core';
import { invoke } from '@tauri-apps/api/core';
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
    
    clipboardItems = [
        'Sample clipboard item 1',
        'Sample clipboard item 2', 
        'Sample clipboard item 3'
    ];

    ngOnInit() {
        // invoke<string[]>('get_clipboard_items').then((items) => {
        //     this.clipboardItems = items;
        // });
    }

    toggleBookmark(item: string) {
        invoke('toggle_bookmark', { item });
    }

    deleteItem(item: string) {
        invoke('delete_clipboard_item', { item });
    }
}
