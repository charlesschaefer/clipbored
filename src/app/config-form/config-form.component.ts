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
import { BookmarkListComponent } from '../bookmark-list/bookmark-list.component';

@Component({
  selector: 'app-config-form',
  standalone: true,
  imports: [ReactiveFormsModule, CommonModule, BookmarkListComponent],
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