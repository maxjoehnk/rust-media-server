import { NgModule } from '@angular/core';
import { CommonModule } from '@angular/common';
import { LibraryComponent } from './library.component';
import { SharedModule } from '../shared/shared.module';
import { RouterModule, Routes } from '@angular/router';
import { AlbumComponent } from './album/album.component';
import { AlbumResolver } from './album/album.resolver';
import { LibraryService } from './library.service';

const routes: Routes = [
    {
        path: 'library',
        component: LibraryComponent
    },
    {
        path: '',
        redirectTo: '/library',
        pathMatch: 'full'
    },
    {
        path: 'library/albums/:id',
        component: AlbumComponent,
        resolve: {
            album: AlbumResolver
        }
    }
];

@NgModule({
    imports: [
        CommonModule,
        SharedModule,
        RouterModule.forChild(routes)
    ],
    declarations: [
        LibraryComponent,
        AlbumComponent
    ],
    exports: [
        LibraryComponent
    ],
    providers: [
        AlbumResolver,
        LibraryService
    ]
})
export class LibraryModule {
}
