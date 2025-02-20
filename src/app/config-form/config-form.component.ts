import { CommonModule } from '@angular/common';
import { Component, effect, signal, WritableSignal } from '@angular/core';
import {
    FormBuilder,
    FormGroup,
    ReactiveFormsModule,
    Validators,
} from '@angular/forms';
import { invoke } from '@tauri-apps/api/core';
import { AppConfig } from '../app-config.model';
import { CardModule } from 'primeng/card';
import { IftaLabelModule } from 'primeng/iftalabel';
import { InputTextModule } from 'primeng/inputtext';
import { InputNumberModule } from 'primeng/inputnumber';
import { PanelModule } from 'primeng/panel';
import { FieldsetModule } from 'primeng/fieldset';
import { CheckboxModule } from 'primeng/checkbox';
import { ButtonModule } from 'primeng/button';
import { InputSwitchModule } from 'primeng/inputswitch';

import { BookmarkListComponent } from '../bookmark-list/bookmark-list.component';
import { ClipboardListComponent } from "../clipboard-list/clipboard-list.component";

@Component({
    selector: 'app-config-form',
    standalone: true,
    imports: [
        ReactiveFormsModule,
        CommonModule,
        BookmarkListComponent,
        CardModule,
        IftaLabelModule,
        InputTextModule,
        InputNumberModule,
        PanelModule,
        FieldsetModule,
        CheckboxModule,
        ButtonModule,
        InputSwitchModule,
        ClipboardListComponent
    ],
    templateUrl: './config-form.component.html',
    styleUrl: './config-form.component.css',
})
export class ConfigFormComponent {
    configForm: FormGroup;
    config = signal<AppConfig>({
        maxItems: 10,
        openShortcut: 'Ctrl+Shift+V',
        bookmarkShortcut: 'Ctrl+Shift+B',
        startMinimized: false
    });
    tempShortcutValue = '';

    isSettingsOpen = false;

    MODIFIERS = ['Ctrl', 'Shift', 'Alt', 'Super'];
    SUPER_MODIFIERS = ['Cmd', 'CmdLeft', 'CmdRight', 'Super', 'Meta', 'MetaLeft', 'MetaRight'];
    DENIED_KEYS = [
        'Delete', 'Backspace', 'Insert', 'CapsLock', 'Escape', 'NumLock', 'Home',
        'End', 'PageDown', 'PageUp', 'Backspace', 'Tab', 'ArrowUp', 'ArrowDown',
        'ArrowLeft', 'ArrowRight', 'AltRight', 'ContextMenu'
    ];

    constructor(private fb: FormBuilder) {
        this.configForm = this.fb.group({
            maxItems: [10, [Validators.required, Validators.min(1)]],
            openShortcut: ['Ctrl+Shift+V', Validators.required],
            bookmarkShortcut: ['Ctrl+Shift+V', Validators.required],
            startMinimized: [false],
        });

        this.loadConfig();

        effect(() => {
            const currentConfig = this.config();
            this.configForm.patchValue(currentConfig, { emitEvent: false }); // Important: prevent infinite loop
        });

        this.configForm.valueChanges.subscribe(values => {
            this.config.set(values);
        });
    }

    async loadConfig() {
        const loadedConfig = await invoke<AppConfig>('get_config');
        if (loadedConfig) {
            this.config.set(loadedConfig);
        }
    }

    onFocus(event: FocusEvent) {
        const target = event.target as HTMLInputElement;
        this.tempShortcutValue = target.value;
        target.value = '';
    }

    onBlur(event: FocusEvent) {
        const target = event.target as HTMLInputElement;
        if (target.value === '') {
            const controlName = target.id === 'openShortcut' ? 'openShortcut' : 'bookmarkShortcut';
            this.configForm.get(controlName)?.setValue(this.tempShortcutValue);
        }
    }

    onKeyDown(event: KeyboardEvent) {
        console.log("KeyDown: ", event.code);

        if (this.DENIED_KEYS.includes(event.code)) {
            this.keyUp(event);
            return; // Let the default behavior happen
        }
        if (event.key !== 'Shift') {
            event.preventDefault();
        }
        const target = event.target as HTMLInputElement;
        let key = event.key;
        if (key === ' ') {
            key = 'Space';
        }
        console.log("Key: ", key);
        const pressedKeys = [];

        if (event.ctrlKey) {
            pressedKeys.push('Ctrl');
        }
        if (event.metaKey) {
            console.log("MetaKey: ", event.metaKey);
            pressedKeys.push('Super');
            if (this.SUPER_MODIFIERS.includes(key)) {
                key = 'Super';
            }
        }
        if (event.shiftKey) {
            pressedKeys.push('Shift');
        }
        if (event.altKey || key.toLowerCase() == 'option') {
            pressedKeys.push('Alt');
        }

        if (!['Control', 'Shift', 'Alt', 'Super'].includes(key)) {
            pressedKeys.push(key.length === 1 ? key.toUpperCase() : key);
        }

        const shortcutString = pressedKeys.join('+');
        target.value = shortcutString;

        const controlName = target.id === 'openShortcut' ? 'openShortcut' : 'bookmarkShortcut';
        this.configForm.get(controlName)?.setValue(shortcutString);
    }

    keyUp(event: KeyboardEvent) {
        const target = event.target as HTMLInputElement;
        const controlName = target.id === 'openShortcut' ? 'openShortcut' : 'bookmarkShortcut';
        const currentValue = this.configForm.get(controlName)?.value;

        const keys = currentValue.split('+');
        // Check if the current value contains only modifier keys
        const onlyModifierKeys = keys.every((part: any) => this.MODIFIERS.includes(part));

        // Check if the value contains at least one modifiers, avoiding shortcuts 
        // like "A" instead of "Ctrl+A"
        const hasModifiers = keys.some((part: any) => this.MODIFIERS.includes(part));

        if (onlyModifierKeys || !hasModifiers) {
            this.configForm.get(controlName)?.setValue('');
            target.value = '';
        }
    }

    saveConfig(): void {
        if (this.configForm.valid) {
            console.log(this.config());
            invoke('set_config', { config: this.config() }).then(() => {
                console.log('Settings Saved');
                invoke('hide_window');
            });
        } else {
            console.log('Form is invalid');
            // Optionally, mark all fields as touched to show validation errors
            this.configForm.markAllAsTouched();
        }
    }
}
