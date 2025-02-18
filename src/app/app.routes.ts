import { Routes } from "@angular/router";
import { AppComponent } from "./app.component";
import { ConfigFormComponent } from "./config-form/config-form.component";

export const routes: Routes = [
    { path: 'config', component: ConfigFormComponent },
    { path: '', redirectTo: '/config', pathMatch: 'full' }
];