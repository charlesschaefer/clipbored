import { NO_ERRORS_SCHEMA } from '@angular/core';
import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ReactiveFormsModule } from '@angular/forms'; // Import ReactiveFormsModule
import { BookmarkListComponent } from '../bookmark-list/bookmark-list.component'; // Mock if needed
import { ConfigFormComponent } from './config-form.component';

describe('ConfigFormComponent', () => {
  let component: ConfigFormComponent;
  let fixture: ComponentFixture<ConfigFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ReactiveFormsModule, ConfigFormComponent], // Add ReactiveFormsModule here
      declarations: [],  // We can often leave this empty in standalone components
      providers: [],      // Add providers if your component depends on services
      schemas: [NO_ERRORS_SCHEMA] //  Ignore unknown elements like <app-bookmark-list>

    })
      .compileComponents();

    fixture = TestBed.createComponent(ConfigFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should initialize the form with default values', () => {
    expect(component.configForm.get('maxItems')?.value).toBe(10);
    expect(component.configForm.get('keyboardShortcut')?.value).toBe('Ctrl+Shift+V');
    expect(component.configForm.get('maxBookmarkedItems')?.value).toBe(5);
    expect(component.configForm.get('startMinimized')?.value).toBe(false);
  });

  it('should mark form as invalid if maxItems is empty', () => {
    component.configForm.get('maxItems')?.setValue('');
    expect(component.configForm.get('maxItems')?.valid).toBeFalse();
  });

  it('should mark form as invalid if maxItems is less than 1', () => {
    component.configForm.get('maxItems')?.setValue(0);
    expect(component.configForm.get('maxItems')?.valid).toBeFalse();
  });

  it('should mark form as invalid if keyboardShortcut is empty', () => {
    component.configForm.get('keyboardShortcut')?.setValue('');
    expect(component.configForm.get('keyboardShortcut')?.valid).toBeFalse();
  });

  it('should mark form as invalid if maxBookmarkedItems is empty', () => {
    component.configForm.get('maxBookmarkedItems')?.setValue('');
    expect(component.configForm.get('maxBookmarkedItems')?.valid).toBeFalse();
  });

  it('should mark form as invalid if maxBookmarkedItems is less than 1', () => {
    component.configForm.get('maxBookmarkedItems')?.setValue(0);
    expect(component.configForm.get('maxBookmarkedItems')?.valid).toBeFalse();
  });

  it('should save config if form is valid', () => {
    const invokeSpy = spyOn(window, 'invoke').and.returnValue(Promise.resolve());
    component.saveConfig();
    expect(invokeSpy).toHaveBeenCalledWith('set_config', { config: component.config() });
  });
});