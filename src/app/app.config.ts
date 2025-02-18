import { ApplicationConfig } from "@angular/core";
import { provideRouter } from "@angular/router";
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { providePrimeNG } from 'primeng/config';
import Aura from '@primeng/themes/aura';

import { routes } from "./app.routes";
import { AppTheme } from "./app.theme";

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideAnimationsAsync(),
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
