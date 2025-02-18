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

import { BookmarkListComponent } from '../bookmark-list/bookmark-list.component';

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
    console.log(event.code);
    if (event.code === 'Tab') {
      this.keyUp(event);
      return; // Let the default behavior happen
    }
    if (event.key !== 'Shift') {
      event.preventDefault();
    }
    const target = event.target as HTMLInputElement;
    let key = event.key;
    if (key === 'Control') {
      key = 'Ctrl';
    } else if (key === ' ') {
      key = 'Space';
    }
    const pressedKeys = [];

    if (event.ctrlKey || event.metaKey) {
      pressedKeys.push('Ctrl');
    }
    if (event.shiftKey) {
      pressedKeys.push('Shift');
    }
    if (event.altKey) {
      pressedKeys.push('Alt');
    }
    if (!['Control', 'Shift', 'Alt', 'Meta'].includes(key)) {
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

      // Check if the current value contains only modifier keys
      const onlyModifierKeys = currentValue.split('+').every((part: any) =>
          ['Ctrl', 'Shift', 'Alt', 'Meta', 'Win', 'Power', 'Cmd'].includes(part)
      );

      if (onlyModifierKeys) {
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