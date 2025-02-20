import { ApplicationConfig } from "@angular/core";
import { provideRouter } from "@angular/router";
import { provideAnimations } from '@angular/platform-browser/animations';
import { providePrimeNG } from 'primeng/config';
import Aura from '@primeng/themes/aura';

import { routes } from "./app.routes";
import { AppTheme } from "./app.theme";

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideAnimations(),
    providePrimeNG({
      theme: {
        preset: AppTheme,
        options: {
          cssLayer: false,
          darkModeSelector: false || 'none'
      }
      }
    }),
  ],
};
