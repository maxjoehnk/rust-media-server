import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LibraryComponent } from './library.component';
import { SharedModule } from '../shared/shared.module';
import { RouterModule, Routes } from '@angular/router';

const routes: Routes = [
    {
        path: 'library',
        component: LibraryComponent
    },
    {
        path: '',
        redirectTo: '/library',
        pathMatch: 'full'
    }
];

@NgModule({
    imports: [
        CommonModule,
        SharedModule,
        RouterModule.forChild(routes)
    ],
    declarations: [
        LibraryComponent
    ],
    exports: [
        LibraryComponent
    ]
})
export class LibraryModule {
}
