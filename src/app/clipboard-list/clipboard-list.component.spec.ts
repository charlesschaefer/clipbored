import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ClipboardListComponent } from './clipboard-list.component';

describe('ClipboardListComponent', () => {
  let component: ClipboardListComponent;
  let fixture: ComponentFixture<ClipboardListComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ClipboardListComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ClipboardListComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
